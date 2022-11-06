#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use CrabOS::{test_panic_handler, logger, graphic_println, println};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    test_main();
    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}