#![no_std] 
#![no_main] 

use core::panic::PanicInfo;
use crate::vga_buffer::{Color, WRITER};
mod vga_buffer;


#[no_mangle] // don't give a random unique name for `_start` function
pub extern "C" fn _start() -> ! {

    WRITER.lock().set_writer_theme(Color::White, Color::Red);
    graphic_println!("sup {}", "test");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

