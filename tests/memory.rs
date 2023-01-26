#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

const PAGE_SIZE: usize = 0x1000;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

use CrabOS::{
    hlt_loop,
    interrupts::{gdt, idt},
    log::{self, info, LevelFilter},
    memory::{frame_distributer::FrameDistributer, heap, paging, buddy_system::manager::BuddyManager},
    test_panic_handler,
};

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    log::init(LevelFilter::Debug);

    gdt::init();
    idt::init();

    let mut distributer = FrameDistributer::new(&boot_info.memory_map);
    info!("frame distributer initialized");

    let mut mapper = unsafe { paging::init(VirtAddr::new(boot_info.physical_memory_offset)) };
    info!("mapper initialized");

    heap::init(&mut mapper, &mut distributer).expect("heap initialization failed");
    info!("heap initialized");
   
    let mut page_frame_allocator = BuddyManager::new(&mut distributer); 
    let process1_address_space_base = page_frame_allocator.allocate(0x20000 / 2, PAGE_SIZE).unwrap();
    let process2_address_space_base = page_frame_allocator.allocate(0x20000 / 2, PAGE_SIZE).unwrap();
    info!("{:?} allocated", process1_address_space_base);
    info!("{:?} allocated", process2_address_space_base);
    
    page_frame_allocator.deallocate(process1_address_space_base, 0x20000 / 2, PAGE_SIZE);
    page_frame_allocator.deallocate(process2_address_space_base, 0x20000 / 2, PAGE_SIZE);
    
    test_main();
    hlt_loop()
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
