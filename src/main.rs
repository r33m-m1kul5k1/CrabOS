#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]

#![allow(unused)]

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;
#[allow(unused_imports)]
use CrabOS::panic::kernel_panic;

use CrabOS::{
    hlt_loop,
    interrupts::{gdt, idt},
    log::{self, info, LevelFilter},
    memory::{frame_distributer::FrameDistributer, heap, paging, self},
    panic::PanicInfo,
};

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {
    #[cfg(test)]
    test_main();

    log::init(LevelFilter::Debug);

    info!("CrabOS starts initialization sequence");
    gdt::init();
    info!("GDT initialized");

    idt::init();
    info!("IDT initialized");

    memory::init(boot_info);
    info!("finished initializing memory related structures");
    
    hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel_panic(info)
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CrabOS::panic::test_panic_handler(info)
}
