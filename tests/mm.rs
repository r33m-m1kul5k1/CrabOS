#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]



use bootloader::{entry_point, bootinfo::BootInfo};
use x86_64::VirtAddr;
use core::panic::PanicInfo;

use CrabOS::{
    log::{self, LevelFilter, info},
    hlt_loop,
    test_panic_handler,
    memory::{frame_allocator::FrameDistributer, paging, heap},
    interrupts::{gdt, idt},
};

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    
    log::init(LevelFilter::Debug);

    gdt::init();
    idt::IDT.load();
    
    let mut distributer = FrameDistributer::new(&boot_info.memory_map);
    info!("frame distributer initialized");
   
    let mut mapper = unsafe {
        paging::init(VirtAddr::new(boot_info.physical_memory_offset))
    };

    info!("mapper initialized");
    heap::init(&mut mapper, &mut distributer).expect("heap initialization failed");
    info!("heap initialized");

    test_main();
    hlt_loop()
}

#[test_case]
fn basic_allocation() {
    //let _ = Box::new(41);
}
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
