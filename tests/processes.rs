#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use CrabOS::{
    interrupts::{
        gdt::{self},
        idt,
    },
    log,
    memory::{self},
    processes::objects::Process,
    test_panic_handler,
    userland::dummy_process,
};

use ::log::LevelFilter;
use bootloader::BootInfo;

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    log::init(LevelFilter::Debug);
    gdt::init();
    idt::init();

    memory::init(boot_info);

    let dummy_process = unsafe { Process::new(0, dummy_process as *const () as u64) };

    dummy_process.execute();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
