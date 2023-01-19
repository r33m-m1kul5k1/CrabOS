#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;
use core::{panic::PanicInfo};
use CrabOS::{
    hlt_loop,
    log::{self, info, LevelFilter},
    interrupts::{gdt, idt},
    memory::{vmm, frame_allocator::FrameDistributer, heap},
};

entry_point!(kmain);

pub fn kmain(boot_info: &'static BootInfo) -> ! {
    #[cfg(test)]
    test_main();

    log::init(LevelFilter::Debug);

    info!("Starts the initialization sequence");
    gdt::init();
    info!("GDT initialized");

    idt::init();
    info!("IDT initialized");

    
    let mut mapper = unsafe {
        vmm::init(VirtAddr::new(boot_info.physical_memory_offset))
    };
    info!("Paging initialized");
    
    
    let mut frame_distributer = FrameDistributer::new(&boot_info.memory_map);

    heap::init(&mut mapper, &mut frame_distributer).expect("heap initialization failed");
    info!("Heap initialized");

    hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CrabOS::test_panic_handler(info)
}
