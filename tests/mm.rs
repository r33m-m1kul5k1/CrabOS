#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]



use bootloader::{entry_point, bootinfo::BootInfo};
use x86_64::VirtAddr;
use core::panic::PanicInfo;
use CrabOS::{
    log::{LevelFilter, logger},
    hlt_loop,
    test_panic_handler,
    memory::{pmm::FrameDistributer, vmm, heap_management::init_heap},
    interrupts::{gdt, idt},
};

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    
    logger::init(LevelFilter::Debug);

    gdt::init();
    idt::IDT.load();
    
    let mut distributer = FrameDistributer::new(&boot_info.memory_map);
    log::info!("frame distributer initialized");
   
    let mut mapper = unsafe {
        vmm::init(VirtAddr::new(boot_info.physical_memory_offset))
    };

    log::info!("mapper initialized");
    init_heap(&mut mapper, &mut distributer).expect("heap initialization failed");
    log::info!("heap initialized");

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
