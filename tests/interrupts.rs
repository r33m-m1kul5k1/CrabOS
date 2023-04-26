#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::tests::runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use ::log::info;
use bootloader::BootInfo;

use core::panic::PanicInfo;
use CrabOS::{
    hlt_loop,
    interrupts::{gdt, idt},
    log::{self, LevelFilter},
    memory::{self, as_ref, kmap, paging::EntryFlags},
    test_should_panic_handler,
};

const NOT_PRESENT_PAGE: u64 = 0xFFFFFFFF;

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    log::init(LevelFilter::Debug);
    gdt::init();
    idt::init();
    memory::init(boot_info);

    test_main();
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_should_panic_handler(info)
}

#[test_case]
fn check_page_fault() {
    unsafe { kmap(NOT_PRESENT_PAGE, 0, EntryFlags::empty()).unwrap() };
    let ref_to_invalid_addr = as_ref::<u64>(NOT_PRESENT_PAGE);
    info!("accessing a not present page");
    let _ = *ref_to_invalid_addr;
}

#[test_case]
fn check_reserved_stack() {
    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow()
    }
    stack_overflow();
}
