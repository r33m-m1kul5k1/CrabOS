#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use CrabOS::{
    interrupts::{
        gdt::{self, GDT},
        idt,
    },
    log,
    memory::{self, kmalloc, types::FRAME_SIZE},
    processes::objects::Thread,
    test_panic_handler,
    userland::dummy_process,
};

use ::log::LevelFilter;
use bootloader::BootInfo;

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    log::init(LevelFilter::Debug);
    gdt::init();
    idt::init();

    memory::init(boot_info);

    let dummy_thread = Thread::new(
        dummy_process as *const () as u64,
        GDT.1.kernel_code.0,
        GDT.1.kernel_data.0,
        boot_info.physical_memory_offset + kmalloc(FRAME_SIZE, FRAME_SIZE).unwrap(),
    );

    unsafe { dummy_thread.run() }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
