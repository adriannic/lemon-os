#![no_std]
#![no_main]

use core::{hint::spin_loop, panic::PanicInfo};

use kernel::{println, riscv::registers::tp};

const BANNER: &str = "
    ██╗     ███████╗███╗   ███╗ ██████╗ ███╗   ██╗       ██████╗ ███████╗
    ██║     ██╔════╝████╗ ████║██╔═══██╗████╗  ██║      ██╔═══██╗██╔════╝
    ██║     █████╗  ██╔████╔██║██║   ██║██╔██╗ ██║█████╗██║   ██║███████╗
    ██║     ██╔══╝  ██║╚██╔╝██║██║   ██║██║╚██╗██║╚════╝██║   ██║╚════██║
    ███████╗███████╗██║ ╚═╝ ██║╚██████╔╝██║ ╚████║      ╚██████╔╝███████║
    ╚══════╝╚══════╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═══╝       ╚═════╝ ╚══════╝
";

#[no_mangle]
extern "C" fn main() -> ! {
    let id = tp::read();

    if id != 0 {
        loop {
            spin_loop();
        }
    }

    // Clear screen
    println!("\x1b[2J\x1b[H");

    // Display banner in gold
    println!("\x1b[38;5;178m{}\x1b[0m", BANNER);

    loop {
        spin_loop();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Print panics in red
    println!("\x1b[38;5;196m{}\x1b0m", info);
    loop {
        spin_loop();
    }
}
