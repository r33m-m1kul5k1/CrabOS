#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]




use bootloader::bootinfo::BootInfo;
use core::panic::PanicInfo;
use CrabOS::{
    log::{LevelFilter, logger},
    hlt_loop,
    test_panic_handler,
    alloc::boxed::Box,
};

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    
    logger::init(LevelFilter::Debug);
    let _ = Box::new(41);
    log::info!("It did not crash!");
    test_main();
    hlt_loop()
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
