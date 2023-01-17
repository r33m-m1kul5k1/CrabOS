#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]


use bootloader::bootinfo::BootInfo;
use x86_64::VirtAddr;
use x86_64::structures::paging::FrameAllocator;
use core::panic::PanicInfo;
use CrabOS::{
    log::{LevelFilter, logger},
    hlt_loop,
    test_panic_handler,
    memory::{pmm::FrameDistributer, vmm, heap_management::init_heap},
    interrupts::{gdt, idt},
};


#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    
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
    
    let mut distributer = FrameDistributer::new(&boot_info.memory_map);

    for _ in 1..20 {
        log::debug!(
            "frame {:?} allocated",
            distributer.allocate_frame().unwrap()
        );
    }

    while let Some(region) = distributer.get_region() {
        log::debug!("region: {:?}", region);
        // check if the region size is in a power of two :)
        assert_eq!(region.size.count_ones(), 1);
    }
    
    // //let _ = Box::new(41);
    log::info!("It did not crash!");
    test_main();
    hlt_loop()
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
