use crate::{
    params::{NCPU, STACK_PAGE_NUM},
    start::start,
};
use core::arch::asm;

/// Struct representing the kernel's stack.
#[repr(C, align(16))]
struct Stack([u8; 4096 * STACK_PAGE_NUM * NCPU]);

/// The kernel's stack.
#[no_mangle]
static mut STACK0: Stack = Stack([0; 4096 * STACK_PAGE_NUM * NCPU]);

/// The kernel's entry point. It's losated at position 0x8000_0000 in memory as detailed in
/// `kernel.ld`.
///
/// # Safety
///
/// This funtion is marked as unsafe because it uses inline assembly, which cannot be checked by
/// rust's compiler.
#[link_section = ".text.init"]
#[no_mangle]
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
