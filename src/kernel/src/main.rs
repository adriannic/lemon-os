#![no_std]
#![no_main]

use core::{hint::spin_loop, panic::PanicInfo};

#[no_mangle]
fn _start() -> ! {
    loop {
        spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        spin_loop();
    }
}
