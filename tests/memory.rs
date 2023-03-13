#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

const PAGE_SIZE: usize = 0x1000;


use alloc::{boxed::Box, vec::Vec};
use bootloader::{bootinfo::BootInfo, entry_point};

use core::{panic::PanicInfo};

use CrabOS::{
    hlt_loop,
    interrupts::{gdt, idt},
    log::{self, info, LevelFilter},
    memory::{
        self, as_mut_ref,
        buddy_system::manager::{BuddyManager, KERNEL_ALLOCATOR},
        frame_distributer::{FrameAllocator, FrameDistributer},
        heap, kfree, kmalloc,
        mapper::{Mapper, KERNEL_MAPPER},
        paging::{self, mmap},
        paging::{get_cr3, Entry, EntryFlags, Table},
        types::FRAME_SIZE,
    },
    test_panic_handler,
};

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    log::init(LevelFilter::Info);

    gdt::init();
    idt::init();

    memory::init(boot_info);
    let physical_addr = kmalloc(FRAME_SIZE, FRAME_SIZE).unwrap();
    kfree(physical_addr, FRAME_SIZE, FRAME_SIZE);
    let physical_addr = kmalloc(FRAME_SIZE, FRAME_SIZE).unwrap();

    test_main();
    hlt_loop()
}

#[test_case]
fn entry_test() {
    let mut entry = Entry::new();

    entry.set_entry(
        0x8000,
        EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER,
    );

    info!("Created {:#x?}", entry);
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
