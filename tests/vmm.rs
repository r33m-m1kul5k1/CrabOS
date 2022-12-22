#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::BootInfo;
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use CrabOS::{test_should_panic_handler, vmm, hlt_loop};

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    
    let _offset_page_table = unsafe {
         vmm::init(VirtAddr::new(boot_info.physical_memory_offset))
    };
    

    test_main();

    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_should_panic_handler(info)
}
