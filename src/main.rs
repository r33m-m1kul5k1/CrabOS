#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(stmt_expr_attributes)]

mod interrupts;
mod logger;
mod serial;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use interrupts::{gdt, idt};

entry_point!(kmain);

pub fn kmain(_boot_info: &'static BootInfo) -> ! {
    logger::init(log::LevelFilter::Info);

    println!("(\\/) (°,,,,°) (\\/)");
    logger::info!("Starts the initialization sequence");

    logger::info!("---Global Descriptor Table and the kernel's Segments");
    gdt::init();
    logger::info!("---Interrupt Descriptor Table");
    idt::IDT.load();
    
    fn stack_overflow() {
        stack_overflow()
    }
    stack_overflow();

    logger::debug!("got to the kmain's end");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
