use crate::{memlayout::STACK_PAGE_NUM, param::NCPU, start::start};
use core::arch::asm;

#[repr(C, align(16))]
struct Stack([u8; 4096 * STACK_PAGE_NUM * NCPU]);

#[no_mangle]
static mut STACK0: Stack = Stack([0; 4096 * STACK_PAGE_NUM * NCPU]);

#[link_section = ".text.init"]
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        "la sp, STACK0",
        "li a0, 4096 * {ssz}",
        "csrr a1, mhartid",
        "addi a1, a1, 1",
        "mul a0, a0, a1",
        "add sp, sp, a0",
        ssz = const STACK_PAGE_NUM,
    );
    start();
}
