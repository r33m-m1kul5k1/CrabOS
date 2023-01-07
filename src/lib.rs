//! This library goal is to expose the OS api to the test directory for integration tests
//! Moreover this library implements basic functionalities for testing
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![feature(generic_const_exprs)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![allow(nonstandard_style)]
#![allow(incomplete_features)]

pub extern crate alloc;
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

/// Exits qemu by writing the exit code to the debug exit port
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(ISA_DEBUG_EXIT_PORT);
        port.write(exit_code as u32);
    }
}

///  Halts the CPU infinitely
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
 
pub trait Testable {
    fn run(&self) -> ();
}

/// Defines a test function which have a run method
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

/// Run the array of #[test_case] functions, and implement for them the `Testable` trait
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// Panic handler for integration testing
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Panic handler that expects to be called
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
