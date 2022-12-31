#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::boxed::Box;

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    
    let x = Box::new(41);
    println!("It did not crash!");
    test_main();
    hlt_loop()
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_should_panic_handler(info)
}
