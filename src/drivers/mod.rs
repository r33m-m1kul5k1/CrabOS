
#[macro_export]
macro_rules! graphic_print {
    ($($arg:tt)*) => {
        $crate::drivers::vga::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! graphic_println {
    () => {
        $crate::graphic_print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::graphic_print!("{}\n", format_args!($($arg)*))
    };
}


#[macro_export]
macro_rules! serial_print {
    // tt -> token tree (() / [] / {})
    ($($arg:tt)*) => {
        $crate::drivers::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    
    () => {
        use crate::print;
        $crate::print!("\n");
    };
    ($fmt:expr) => {
        $crate::serial_print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::serial_print!(concat!($fmt, "\n"), $($arg)+);
    }
}



pub mod serial;
pub mod vga;