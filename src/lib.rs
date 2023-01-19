//! This library goal is to expose the OS api to the test directory for integration tests
//! Moreover this library implements basic functionalities for testing
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![allow(non_snake_case)]


pub extern crate alloc;

/// note that `pub` keyword makes the modules declaration accessible to external crates
pub mod interrupts;
pub mod log;
pub mod drivers;
pub mod memory;
pub mod panic;
pub mod tests;


pub use core::panic::PanicInfo;
pub use panic::{hlt_loop, test_panic_handler, test_should_panic_handler};
