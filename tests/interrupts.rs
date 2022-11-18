#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use CrabOS::{test_should_panic_handler, interrupts::{gdt, idt}};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {

    gdt::init();
    idt::IDT.load();
    test_main();
    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_should_panic_handler(info)
}

#[test_case]
fn basic_print() {
    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow()
    }
    stack_overflow();
}