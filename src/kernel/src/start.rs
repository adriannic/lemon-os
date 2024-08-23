use core::hint::spin_loop;

use crate::{memlayout::STACK_PAGE_NUM, param::NCPU};

#[repr(C, align(16))]
struct Stack([u8; 4096 * STACK_PAGE_NUM * NCPU]);

#[no_mangle]
static mut STACK0: Stack = Stack([0; 4096 * STACK_PAGE_NUM * NCPU]);

pub unsafe fn start() -> ! {
    loop {
        spin_loop();
    }
}
