#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use CrabOS::{
    hlt_loop,
    interrupts::{gdt, idt},
    log::{self, LevelFilter},
    memory::{self},
    test_should_panic_handler,
};

entry_point!(main);
pub fn main(boot_info: &'static BootInfo) -> ! {
    log::init(LevelFilter::Debug);
    gdt::init();
    idt::init();
    memory::init(boot_info);

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
