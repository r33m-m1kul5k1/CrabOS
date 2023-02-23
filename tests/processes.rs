#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use CrabOS::{
    userland::dummy_process,
    test_should_panic_handler, // hlt_loop,
    log::logger::init
};

use log::LevelFilter;



#[no_mangle]
pub extern "C" fn _start() -> ! {
    init(LevelFilter::Info);
    dummy_process();
    // test_main();
    // hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_should_panic_handler(info)
}

/* 
#[test_case]
fn check_context_switch() {
    
}
*/