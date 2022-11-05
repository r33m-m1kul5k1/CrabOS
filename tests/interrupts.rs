#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use CrabOS::{interrupts, test_panic_handler, print};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    CrabOS::print!("stack_overflow::stack_overflow...\t");

    interrupts::gdt::init();
    // trigger a stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}