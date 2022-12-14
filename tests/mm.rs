#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::BootInfo;
use CrabOS::{memory::pmm::FrameDistributer, hlt_loop, test_panic_handler};

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {

    let mut distributer = FrameDistributer::new(&boot_info.memory_map);
    distributer.get_unused_region();
    test_main();
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[test_case]
fn check_frame_distributer_initialization() {
    
}
