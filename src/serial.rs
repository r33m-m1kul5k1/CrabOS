/*
The uart module goal is to implement`println!` macro with a static serial
that will be guarded using a spinlock.
*/

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8;

lazy_static! {
    pub static ref SERIAL: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    // tt -> token tree (() / [] / {})
    ($($arg:tt)*) => {
        crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        crate::print!("\n");
    };
    ($fmt:expr) => {
        crate::print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)+) => {
        crate::print!(concat!($fmt, "\n"), $($arg)+);
    }
}
