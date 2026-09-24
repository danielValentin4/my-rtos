

#[repr(C, align(8))]
#[derive(Clone)]
pub struct TaskStack(pub [u8; 1024]);

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
}

pub struct Task {
    //pub stack: TaskStack,
    pub priority: TaskPriority,
    pub sp: *mut u32,
    pub state: TaskState,
}

fn init_stack(stack: &mut TaskStack, entry: fn() -> !) -> *mut u32 {
    let stack_top = unsafe { stack.0.as_mut_ptr().add(stack.0.len()) as *mut u32 };

    let mut sp = (stack_top as usize & !0b111) as *mut u32;

    unsafe {
        sp = sp.sub(1);
        *sp = 0x01000000; // xPSR
        sp = sp.sub(1);
        *sp = entry as usize as u32; // pc
        sp = sp.sub(1);
        *sp = 0xFFFFFFFD;
        sp = sp.sub(1);
        *sp = 0;
        sp = sp.sub(1);
        *sp = 0;
        sp = sp.sub(1);
        *sp = 0;
        sp = sp.sub(1);
        *sp = 0;
        sp = sp.sub(1);
        *sp = 0;

        for _ in 0..8 {
            sp = sp.sub(1);
            *sp = 0;
        }
    }

    sp
}

pub fn create_task(stack : &mut TaskStack, entry: fn() -> !, priority: TaskPriority) -> Task {
    let mut task = Task {
        //stack: TaskStack([0; 512]),
        priority,
        sp: core::ptr::null_mut(),
        state: TaskState::Ready,
    };

    task.sp = init_stack(stack, entry);

    task
}
