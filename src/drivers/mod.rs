
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

pub mod vga;