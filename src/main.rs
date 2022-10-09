#![no_std]
#![no_main]

mod uart;

use bootloader::{ BootInfo, entry_point };
use core::panic::PanicInfo;


entry_point!(kmain);

pub fn kmain(_boot_info: &'static BootInfo) -> ! {
    uart::init_uart();
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}