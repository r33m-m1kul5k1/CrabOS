#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]

#![allow(unused)]
extern crate alloc;

const PAGE_SIZE: usize = 0x1000;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::{VirtAddr, registers::control::Cr3};

use CrabOS::{
    hlt_loop,
    interrupts::{gdt, idt},
    log::{self, info, LevelFilter},
    memory::{frame_distributer::FrameDistributer, heap, paging, buddy_system::manager::BuddyManager, mapper::Mapper, paging::{Entry, EntryFlags}},
    test_panic_handler,
};

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    log::init(LevelFilter::Debug);

    gdt::init();
    idt::init();

    info!("Memory map: {:#?}", boot_info.memory_map);
    
    let mut _distributer = FrameDistributer::new(&boot_info.memory_map);
    info!("frame distributer initialized");

    
    test_main();
    hlt_loop()
}

#[test_case]
fn load_cr3_and_flush_tlb() {
    let mapper = Mapper::new(Cr3::read().0.start_address().as_u64());
    unsafe {
        mapper.load_cr3();
    }

    info!("loading cr3 successfully");
}

#[test_case]
fn entry_test() {
    let mut entry = Entry::new();

    entry.set_entry(0x8000, EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER);

    info!("Created {:#x?}", entry);
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
