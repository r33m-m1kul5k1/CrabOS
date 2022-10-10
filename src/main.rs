#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod serial;
mod logger;
mod interrupts;

use interrupts::idt;
use bootloader::{ BootInfo, entry_point };
use core::panic::PanicInfo;

entry_point!(kmain);

pub fn kmain(_boot_info: &'static BootInfo) -> ! {

    logger::init(log::LevelFilter::Debug);
    println!("(\\/) (°,,,,°) (\\/)");
    idt::IDT.load();

    #[allow(unconditional_panic)]
    let _ = 1 / 0;
    

    logger::debug!("got to the kmain's end");
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}