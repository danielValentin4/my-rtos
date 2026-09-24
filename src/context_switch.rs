use core::arch::asm;

#[unsafe(no_mangle)]
pub static mut CURR_TASK_SP_PTR: *mut *mut u32 = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut NEXT_TASK_SP: *mut u32 = core::ptr::null_mut();

core::arch::global_asm!(r#"
    .section .text.PendSV
    .global PendSV
    .thumb_func
PendSV:
    mrs r0, psp
    stmdb r0!, {{r4-r11}}
    ldr r1, =CURR_TASK_SP_PTR
    ldr r1, [r1]
    str r0, [r1]
    ldr r1, =NEXT_TASK_SP
    ldr r0, [r1]
    ldmia r0!, {{r4-r11}}
    msr psp, r0
    isb
    ldr lr, =0xFFFFFFFD
    bx lr
"#);

pub fn start_first_task(sp: *mut u32) -> ! {
    unsafe {
        asm!(
                "msr psp, {0}",
                "movs r0, #2",
                "msr control, r0",
                "isb",
                "pop {{r4-r11}}",
                "pop {{r0-r3, r12, lr}}",
                "pop {{pc}}",
                in(reg) sp,
                options(noreturn)
        );
    }
}
