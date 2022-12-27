//! this library goal is to expose the OS api to the test directory for integration tests
//! moreover this library implements basic functionalities for testing
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![allow(nonstandard_style)] 

use core::panic::PanicInfo;
/// note that `pub` keyword makes the modules declaration accessible to external crates
pub mod interrupts;
pub mod log;
pub mod drivers;
pub mod memory;

const ISA_DEBUG_EXIT_PORT: u16 = 0xf4;
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(ISA_DEBUG_EXIT_PORT);
        port.write(exit_code as u32);
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

pub fn test_should_panic_handler(info: &PanicInfo) -> ! {
    println!("[Success]\n");
    println!("Panic info: {}\n", info);
    exit_qemu(QemuExitCode::Success);
    hlt_loop();
}

#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    test_main();

    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
