#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]




use bootloader::bootinfo::BootInfo;
use x86_64::VirtAddr;
use core::panic::PanicInfo;
use CrabOS::{
    log::{LevelFilter, logger},
    hlt_loop,
    test_panic_handler,
    alloc::boxed::Box, memory::{pmm::FrameDistributer, vmm, heap_management::init_heap},
};

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    
    logger::init(LevelFilter::Debug);
    let mut distributer = FrameDistributer::new(&boot_info.memory_map);
    let mut mapper = unsafe {
        vmm::init(VirtAddr::new(boot_info.physical_memory_offset))
    };

    init_heap(&mut mapper, &mut distributer);
    
    let _ = Box::new(41);
    log::info!("It did not crash!");
    test_main();
    hlt_loop()
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
