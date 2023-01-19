#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![allow(non_snake_case)]

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;
use CrabOS::{
    log::{self, info, LevelFilter},
    interrupts::{gdt, idt},
    memory::{vmm, frame_allocator::FrameDistributer, heap},
    panic::{kernel_panic, PanicInfo},
    hlt_loop,
};

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {

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


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel_panic(info)
}
