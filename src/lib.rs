//! This library goal is to expose the OS api to the test directory for integration tests
#![no_std]
#![no_main]
#![allow(non_snake_case)]

#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![feature(const_mut_refs)]

#![test_runner(crate::tests::runner)]
#![reexport_test_harness_main = "test_main"]

pub extern crate alloc;

pub mod drivers;
/// note that `pub` keyword makes the modules declaration accessible to external crates
pub mod interrupts;
pub mod log;
pub mod memory;
pub mod panic;
pub mod tests;

pub use core::panic::PanicInfo;
pub use panic::{hlt_loop, test_panic_handler, test_should_panic_handler};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic::test_panic_handler(info)
}
