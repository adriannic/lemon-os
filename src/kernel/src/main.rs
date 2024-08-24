#![no_std]
#![no_main]

use core::arch::asm;
use core::hint::spin_loop;
use core::panic::PanicInfo;

use kernel::memlayout::STACK_PAGE_NUM;
use kernel::println;

const BANNER: &str = "
    ██╗     ███████╗███╗   ███╗ ██████╗ ███╗   ██╗       ██████╗ ███████╗
    ██║     ██╔════╝████╗ ████║██╔═══██╗████╗  ██║      ██╔═══██╗██╔════╝
    ██║     █████╗  ██╔████╔██║██║   ██║██╔██╗ ██║█████╗██║   ██║███████╗
    ██║     ██╔══╝  ██║╚██╔╝██║██║   ██║██║╚██╗██║╚════╝██║   ██║╚════██║
    ███████╗███████╗██║ ╚═╝ ██║╚██████╔╝██║ ╚████║      ╚██████╔╝███████║
    ╚══════╝╚══════╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═══╝       ╚═════╝ ╚══════╝
";

#[allow(clippy::missing_safety_doc)]
pub unsafe fn main() -> ! {
    let cpuid = mhartid();
    if cpuid != 0 {
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
    main();
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn mhartid() -> usize {
    let id;
    asm!("csrr {0}, mhartid", out(reg) id);
    id
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        spin_loop();
    }
}
