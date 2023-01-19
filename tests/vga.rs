#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use CrabOS::{
    graphic_println,
    drivers::vga::{WRITER, Color},
    test_panic_handler,
    hlt_loop, 
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    WRITER
        .lock()
        .set_writer_theme(Color::LightRed, Color::Black);
    test_main();
    hlt_loop();
}


#[test_case]
fn basic_print() {
    graphic_println!(r"(\/) (°,,,,°) (\/)");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
