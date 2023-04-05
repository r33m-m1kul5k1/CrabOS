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
    memory::{
        self, as_addr, get_physical_addr, kfree, kmalloc, kmap,
        paging::EntryFlags, types::FRAME_SIZE, as_ref, get_linear_addr,
    },
    test_panic_handler,
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
fn test_mapping() {
    let first_arr = [1, 2, 3, 4];
    let arr_address = as_addr(&first_arr);

    let arr_page = (arr_address >> 12) << 12;
    let arr_page_frame = get_physical_addr(arr_page).unwrap();
    let arr_page = get_linear_addr(arr_page_frame);

    unsafe {
        kmap(
            arr_page,
            arr_page_frame,
            EntryFlags::PRESENT | EntryFlags::USER,
        ).unwrap();
    }

    let first_arr = *as_ref::<[i32; 4]>(arr_address);
    let second_arr = *as_ref::<[i32; 4]>(arr_page | (arr_address & 0xFFF));

    info!("first arr: {:?}\nsecond arr: {:?}", first_arr, second_arr);
    assert!(first_arr == second_arr)
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
