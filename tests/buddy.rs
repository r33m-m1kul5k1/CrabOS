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
    
    logger::init(LevelFilter::Debug);

    let mut distributer = FrameDistributer::new(&boot_info.memory_map);

    // manages 10 frames 
    let mut buddy = Buddy::<4>::new(distributer.get_region().unwrap(), 0x1000);

    log::info!("order of block under the limit is: {}", buddy.get_order(1).unwrap());
    log::info!("order of block of size 2Kib: {}", buddy.get_order(2* 0x1000).unwrap());

    let first_block = buddy.allocate(4*0x1000, 0x1000).unwrap();
    let second_block = buddy.allocate(4*0x1000, 0x1000).unwrap();
    log::info!("allocated {:?}", first_block);
    log::info!("allocated {:?}", second_block);
    
    test_main();
    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
