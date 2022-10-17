#![no_std] 
#![no_main] 

use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle] // don't give a random unique name for `_start` function
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

