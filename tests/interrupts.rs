#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use CrabOS::{
    log::{self, LevelFilter},
    interrupts::{gdt, idt},
    test_should_panic_handler, hlt_loop,
};



#[no_mangle]
pub extern "C" fn _start() -> ! {

    log::init(LevelFilter::Debug);
    
    gdt::init();
    idt::IDT.load();

    test_main();
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_should_panic_handler(info)
}

#[test_case]
fn check_reserved_stack() {
    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow()
    }
    stack_overflow();
}
