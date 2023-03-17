#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{bootinfo::BootInfo, entry_point};

use core::panic::PanicInfo;

use CrabOS::{
    hlt_loop,
    interrupts::{gdt, idt},
    log::{self, info, LevelFilter},
    memory::{self, kmalloc, types::FRAME_SIZE, kfree}, test_panic_handler,
};

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    log::init(LevelFilter::Debug);

    gdt::init();
    idt::init();

    memory::init(boot_info);

    test_main();
    hlt_loop()
}

#[test_case]
fn kernel_allocations() {
    let address = kmalloc(FRAME_SIZE, FRAME_SIZE).unwrap();
    info!("allocated {:x} of size {:x}", address, FRAME_SIZE);
    kfree(address, FRAME_SIZE, FRAME_SIZE);
    info!("freed {:x} of size {:x}", address, FRAME_SIZE);
}
#[test_case]
fn basic_allocation() {
    let _ = Box::new(41);
}

#[test_case]
fn big_allocation() {
    let mut vec = Vec::<i32>::new();

    for i in 0..100 {
        vec.push(i);
    }

    info!("{:?}", vec);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
