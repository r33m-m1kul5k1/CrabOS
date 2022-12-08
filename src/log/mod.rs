
#[macro_export]
macro_rules! print {
    // tt -> token tree (() / [] / {})
    ($($arg:tt)*) => {
        $crate::log::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    
    () => {
        use crate::print;
        $crate::print!("\n");
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)+);
    }
}



pub mod serial;
pub mod logger;
// everyone that will import the log module will have direct access to the logging functions
pub use log::{debug, error, info, trace, warn};
