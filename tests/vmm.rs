#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::BootInfo;
use x86_64::VirtAddr;
use core::panic::PanicInfo;
use CrabOS::{test_should_panic_handler, vmm};

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    let virtual_add = VirtAddr { 
        addr: boot_info.physical_memory_offset 
    };
    unsafe { vmm::init(virtual_add) };
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_should_panic_handler(info)
}
