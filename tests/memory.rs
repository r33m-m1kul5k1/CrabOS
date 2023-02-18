#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused)]
extern crate alloc;

const PAGE_SIZE: usize = 0x1000;

use ::log::debug;
use alloc::{boxed::Box, vec::Vec};
use bootloader::{bootinfo::BootInfo, entry_point};

use core::{arch::asm, borrow::BorrowMut, panic::PanicInfo};

use CrabOS::{
    hlt_loop,
    interrupts::{gdt, idt},
    log::{self, info, LevelFilter},
    memory::{
        as_mut_ref,
        buddy_system::manager::BuddyManager,
        frame_distributer::{FrameAllocator, FrameDistributer},
        heap,
        mapper::Mapper,
        paging::{self, mmap},
        paging::{get_cr3, Entry, EntryFlags, Table},
    },
    test_panic_handler,
};

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    log::init(LevelFilter::Info);

    gdt::init();
    idt::init();

    debug!("Memory map: {:#?}", boot_info.memory_map);
    debug!(
        "virtual memory offset: 0x{:x}",
        boot_info.physical_memory_offset
    );

    let mut distributer = FrameDistributer::new(&boot_info.memory_map);
    info!("frame distributer initialized");

    let mut mapper = Mapper::new(
        as_mut_ref::<Table>(get_cr3()),
        boot_info.physical_memory_offset,
    );
    let physical_addr = distributer.allocate_frame().unwrap();
    let linear_addr = physical_addr + boot_info.physical_memory_offset;
    info!("mapping 0x{:x} to 0x{:x}", linear_addr, physical_addr);

    unsafe {
        mapper.map(
            linear_addr,
            physical_addr,
            &mut distributer,
            EntryFlags::PRESENT | EntryFlags::WRITABLE,
        )
    }
    let physical_addr = mapper.linear_to_physical(linear_addr).unwrap();

    info!(
        "successfully mapped page 0x{:x} to page frame: 0x{:x}",
        linear_addr, physical_addr
    );

    heap::init(&mut mapper, &mut distributer);

    let mut buddy_manager = BuddyManager::new(&mut distributer);

    // allocates 32 pages for a new dummy process
    let dummy_process = buddy_manager.allocate(0x20000, 0x1000).unwrap();

    info!(
        "allocates 32 pages for a new dummy process at address 0x{:x}",
        dummy_process
    );
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
