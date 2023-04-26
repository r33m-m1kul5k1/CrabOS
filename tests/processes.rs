#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused)]

use core::{panic::PanicInfo, arch::asm};

use CrabOS::{
    interrupts::{
        gdt::{self},
        idt, get_kernel_selectors,
    },
    log,
    memory::{self, kmap, as_addr},
    processes::objects::{Process, Thread},
    test_panic_handler,
    userland::{dummy_process, user_main, logo_print}, syscalls::{self, syscall_handler},
};

use ::log::{LevelFilter, debug, info};
use bootloader::BootInfo;

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    let (cs, ds) = get_kernel_selectors();
    let mut stack_top: u64;

    log::init(LevelFilter::Debug);
    gdt::init();
    idt::init();
    syscalls::init();
    info!("succefully initialized the gdt, idt and syscalls");
    memory::init(boot_info);
    
    unsafe { asm!("mov {}, rsp", out(reg) stack_top)};
    debug!("current stack top: 0x{:x}", stack_top);
    let _dummy_thread = Thread::new(
        logo_print as *const () as u64,
        cs,
        ds,
        stack_top,
    );

    // unsafe { _dummy_thread.run() }

    
    let userland_shell = unsafe { Process::new(0, user_main as *const () as u64) };

    userland_shell.execute();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
