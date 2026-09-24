use crate::context_switch;
use crate::tasks::{Task, TaskPriority, TaskState};
use core::array::from_fn;
use cortex_m::peripheral::SCB;
use cortex_m_semihosting::hprintln;
use cortex_m_semihosting::debug;

const MAX_TASKS: usize = 8;

pub struct Scheduler {
    tasks: [Option<Task>; MAX_TASKS],
    current: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: from_fn(|_| None),
            current: MAX_TASKS - 1,
        }
    }
    
    pub fn show_number_of_tasks(&self) {
        let elements = self.tasks.iter().filter(|x| x.is_some()).count();
        hprintln!("There are : {} tasks waiting to run.", elements);
    }

    pub fn pick_next_task(&mut self) -> *mut u32 {
        for target_priority in [
            TaskPriority::High,
            TaskPriority::Normal,
            TaskPriority::Low,
            TaskPriority::Idle,
        ] {
            for offset in 1..MAX_TASKS {
                let i = (self.current + offset) % MAX_TASKS;
                if let Some(task) = &mut self.tasks[i] && self.current != i {
                    if task.priority == target_priority && task.state == TaskState::Ready {
                        self.current = i;
                        task.state = TaskState::Running;
                        return task.sp;
                    }
                    
                }
            }
        }
        core::ptr::null_mut()
    }

    pub fn add_task(&mut self, task: Task) {
        for i in 0..MAX_TASKS {
            if self.tasks[i].is_none() {
                self.tasks[i] = Some(task);
                break;
            }
        }
    }

    pub fn schedule(&mut self, is_task_done : bool) {
        let cur = self.current;
        let next_sp = self.pick_next_task();
        if next_sp.is_null() {
            return;
        }
        unsafe {
            if let Some(t) = self.tasks.get_mut(cur).and_then(|t| t.as_mut()) {
                context_switch::CURR_TASK_SP_PTR = &mut t.sp;
                t.state = TaskState::Ready;
                if is_task_done {
                    t.state = TaskState::Blocked;
                    self.tasks[cur] = None;
                    self.show_number_of_tasks();
                    if self.tasks.iter().filter(|t| t.is_some()).count() == 0 {
                        hprintln!("No more tasks scheduled.");
                        debug::exit(debug::EXIT_SUCCESS);
                    }
                }
            }
            context_switch::NEXT_TASK_SP = next_sp;
        };            
        SCB::set_pendsv();
        
    }
}
