use core::fmt::{self, Write};

use crate::{memlayout::UART_ADDR, spinlock::Mutex};

static UART: Mutex<Uart> = Mutex::new(Uart::new());

/// Represents the [UART](https://en.wikipedia.org/wiki/Universal_asynchronous_receiver-transmitter) interface.
#[derive(Default)]
pub struct Uart {}

impl Uart {
    pub const fn new() -> Self {
        Self {}
    }
}

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
