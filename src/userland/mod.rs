//! this module includes userland processes


use core::arch::asm;

use crate::log::logger;

// the sign ! `never` means that the func never returns
pub fn dummy_process() -> ! {
    logger::info!("hello from user land :)");
    loop {}
}


pub fn naked_process() -> !{
    unsafe { asm!("mov rax, 1", "hlt", options(noreturn)) }
}
