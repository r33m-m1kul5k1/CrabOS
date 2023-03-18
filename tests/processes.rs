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
    memory::frame_distributer::{FrameAllocator, FrameDistributer},
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
    idt::IDT.load();

    let mut distributer = FrameDistributer::new(&boot_info.memory_map);

    let dummy_thread = Thread::new(
        dummy_process as *const () as u64,
        GDT.1.kernel_code.0,
        GDT.1.kernel_data.0,
        boot_info.physical_memory_offset + distributer.allocate_frame().unwrap(),
    );

    unsafe { dummy_thread.run() }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
