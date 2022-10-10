#![no_std]
#![no_main]

mod serial;
mod logger;

use bootloader::{ BootInfo, entry_point };
use log::{ info, warn, error };
use core::panic::PanicInfo;

entry_point!(kmain);

pub fn kmain(_boot_info: &'static BootInfo) -> ! {

    logger::init(log::LevelFilter::Warn);
    info!("hello log");
    warn!("warning");
    error!("oops");
    
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}