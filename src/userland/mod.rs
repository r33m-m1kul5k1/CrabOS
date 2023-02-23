//! this module includes userland processes

use crate::log::logger;

// the sign ! `never` means that the func never returns
pub fn dummy_process() -> ! {
    logger::info!("hello from user land :)");
    loop {}
}