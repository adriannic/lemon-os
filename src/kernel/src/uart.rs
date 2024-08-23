use core::fmt::{self, Write};

use crate::{memlayout::UART_ADDR, spinlock::Mutex};

pub static UART: Mutex<Uart> = Mutex::new(Uart {});

pub struct Uart {}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let uart = UART_ADDR as *mut u8;
        for c in s.bytes() {
            unsafe {
                *uart = c;
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    UART.lock().write_fmt(args).unwrap();
}