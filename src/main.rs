#![no_std]
#![no_main]

mod context_switch;
mod scheduler;
mod tasks;

use scheduler::Scheduler;
use tasks::{TaskPriority, TaskStack};

use core::ptr::addr_of_mut;
use cortex_m_rt::entry;
use cortex_m_rt::exception;
use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

static mut SCHEDULER: Option<Scheduler> = None;

static mut STACK_A : TaskStack = TaskStack([0; 1024]);
static mut STACK_B : TaskStack = TaskStack([0; 1024]);
static mut STACK_C : TaskStack = TaskStack([0; 1024]);
static mut STACK_D : TaskStack = TaskStack([0; 1024]);



#[exception]
unsafe fn HardFault(_frame : &cortex_m_rt::ExceptionFrame) -> ! {
    hprintln!("HardFault");
    hprintln!("PC : {}", _frame.pc());
    let cfsr = core::ptr::read_volatile(0xE000_ED28 as *const u32);
    hprintln!("CFSR : {}", cfsr);
    loop {}
}

fn task_a() -> ! {
    let mut i : u16 = 0;
    loop {
        if i < 5 {
            i += 1; // loop testing
        }
        core::hint::black_box(i);
        unsafe {
            let sched_ptr = addr_of_mut!(SCHEDULER);
            if let Some(s) = (*sched_ptr).as_mut() {
                if i >= 5 {
                    s.add_task(tasks::create_task(&mut *core::ptr::addr_of_mut!(STACK_D), task_d, TaskPriority::High)); // adding a new task after one just finished
                    s.schedule(true); 
                }
                s.schedule(false);
            }
        }
    }
}

fn task_d() -> ! {
    loop {
        hprintln!("Task D is scheduled.");
        unsafe {
            let sched_ptr = addr_of_mut!(SCHEDULER);
            if let Some(s) = (*sched_ptr).as_mut() {
                s.schedule(false);
            }
        }
    }
}

fn task_b() -> ! {
    loop {
        hprintln!("Task B scheduled now.");
        unsafe {
            let sched_ptr = addr_of_mut!(SCHEDULER);
            if let Some(s) = (*sched_ptr).as_mut() {
                s.schedule(true); // done after just one running
            }
        }
    }
}

fn task_c() -> ! {
    loop {
        hprintln!("Task C scheduled now. ");
        unsafe {
            let sched_ptr = addr_of_mut!(SCHEDULER);
            if let Some(s) = (*sched_ptr).as_mut() {
                s.schedule(false); // constantly running
            }
        }
    }
}

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");

    unsafe {
        let sch_ptr = addr_of_mut!(SCHEDULER);
        *sch_ptr = Some(Scheduler::new());
        let scheduler = (*sch_ptr).as_mut().unwrap();
        scheduler.show_number_of_tasks();
        scheduler.add_task(tasks::create_task(&mut *core::ptr::addr_of_mut!(STACK_A), task_a, TaskPriority::High));
        scheduler.add_task(tasks::create_task(&mut *core::ptr::addr_of_mut!(STACK_B), task_b, TaskPriority::High));
        scheduler.add_task(tasks::create_task(&mut *core::ptr::addr_of_mut!(STACK_C), task_c, TaskPriority::High));

        let first_sp = scheduler.pick_next_task();

        if !first_sp.is_null() {
            hprintln!("First task is starting.");
            context_switch::start_first_task(first_sp);
        } else {
            scheduler.show_number_of_tasks();
            hprintln!("No task registered.");
            loop {}
        }
    }
}
