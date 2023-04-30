#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused)]

use core::{arch::asm, panic::PanicInfo};

use CrabOS::{
    interrupts::{
        gdt,
        get_kernel_selectors, idt,
    },
    log,
    memory::{self, as_addr, kmap, get_physical_addr},
    processes::objects::{Process, Thread},
    test_panic_handler,
    userland::{user_main, LOGO},
};

use ::log::{debug, info, LevelFilter};
use bootloader::{BootInfo, entry_point};

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    let (cs, ds) = get_kernel_selectors();
    let mut stack_top: u64;

    log::init(LevelFilter::Debug);
    gdt::init();
    idt::init();
    
    memory::init(boot_info);


    
    unsafe { asm!("mov {}, rsp", out(reg) stack_top) };
    debug!("stack virtual address {:#x}, physical address: {:#x} ", stack_top, get_physical_addr(stack_top).unwrap());
    let _dummy_thread = Thread::new(user_main as *const () as u64, cs, ds, stack_top);

    // unsafe { _dummy_thread.run() }

    let userland_process = unsafe { Process::new(0, user_main as *const () as u64, LOGO.as_ptr() as u64) };

    userland_process.execute();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
