#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]


use bootloader::bootinfo::BootInfo;
use core::panic::PanicInfo;
use CrabOS::{hlt_loop, log::{LevelFilter, logger}, memory::{pmm::FrameDistributer, buddy::Buddy}, test_panic_handler};

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    
    logger::init(log::LevelFilter::Debug);

    let mut distributer = FrameDistributer::new(&boot_info.memory_map);
    let mut buddy = Buddy::<4>::new(distributer.get_region().unwrap(), 4096);
    buddy.req_size_to_level(1);
    buddy.get_free_block(0);
    test_main();
    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
