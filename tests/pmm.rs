#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::BootInfo;
use core::panic::PanicInfo;
use x86_64::structures::paging::FrameAllocator;
use CrabOS::{hlt_loop, log, memory::pmm::FrameDistributer, test_panic_handler};

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    log::logger::init(log::LevelFilter::Debug);

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
        let region_size = region.end_addr() - region.start_addr();
        assert_eq!(region_size.count_ones(), 1);
    }

    test_main();
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}