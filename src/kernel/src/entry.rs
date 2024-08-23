use core::arch::asm;
use core::hint::unreachable_unchecked;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    asm!("wfi");
    unreachable_unchecked();
}
