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
        self, as_addr, as_ref, get_linear_addr, get_physical_addr, kfree, kmalloc, kmap,
        paging::EntryFlags, types::{PAGE_SIZE, VirtualMemoryRegion}, update_pages_access_policy,
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
        )
        .unwrap();
    }

    let first_arr = *as_ref::<[i32; 4]>(arr_address);
    let second_arr = *as_ref::<[i32; 4]>(arr_page | (arr_address & 0xFFF));

    info!("first arr: {:?}\nsecond arr: {:?}", first_arr, second_arr);
    assert!(first_arr == second_arr)
}

#[test_case]
fn kernel_allocations() {

    info!("-------------------Allocation-------------------\n");
    let process_1 = kmalloc(PAGE_SIZE, PAGE_SIZE).unwrap();
    info!("kernel allocated {:x} of size {:x} to process 1", process_1, PAGE_SIZE);
    info!("-----------------------------------------------\n");
    let process_2 = kmalloc(PAGE_SIZE, PAGE_SIZE).unwrap();
    info!("kernel allocated {:x} of size {:x} to process 2", process_2, PAGE_SIZE);
    info!("-----------------------------------------------\n");

    info!("------------------Deallocation------------------\n");

    kfree(process_1, PAGE_SIZE, PAGE_SIZE);
    info!("freed process 1 memory: {:x}", process_1);
    info!("-----------------------------------------------\n");

    kfree(process_2, PAGE_SIZE, PAGE_SIZE);
    info!("freed process 2 memory: {:x}", process_2);
    info!("-----------------------------------------------\n");

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

#[test_case]
fn linear_address_translation_check() {
    let region =  VirtualMemoryRegion::new(as_addr(&get_linear_addr), 0, 1);
    
    info!("linear address 0x{:x} -> physical address {:#x}", region.first_page(), get_physical_addr(region.first_page()).unwrap());
    unsafe { update_pages_access_policy(region.clone(), EntryFlags::PRESENT | EntryFlags::WRITABLE) };
    info!("after updating the page of physical address {:#x}", get_physical_addr(region.first_page()).unwrap());
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
