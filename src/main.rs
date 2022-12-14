#![no_std]
#![no_main]
#![allow(nonstandard_style)]
#![feature(abi_x86_interrupt)]
#![feature(stmt_expr_attributes)]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use interrupts::{gdt, idt};
use vga_buffer::{Color, WRITER};
use CrabOS::{graphic_println, interrupts, logger, vga_buffer, hlt_loop};

entry_point!(kmain);

pub fn kmain(_boot_info: &'static BootInfo) -> ! {
    
    #[cfg(test)]
    test_main();

    WRITER
        .lock()
        .set_writer_theme(Color::LightRed, Color::Black);
    graphic_println!(
        r"
  $$$$$$\                     $$\        $$$$$$\   $$$$$$\  
 $$  __$$\                    $$ |      $$  __$$\ $$  __$$\ 
 $$ /  \__| $$$$$$\  $$$$$$\  $$$$$$$\  $$ /  $$ |$$ /  \__|
 $$ |      $$  __$$\ \____$$\ $$  __$$\ $$ |  $$ |\$$$$$$\  
 $$ |      $$ |  \__|$$$$$$$ |$$ |  $$ |$$ |  $$ | \____$$\ 
 $$ |  $$\ $$ |     $$  __$$ |$$ |  $$ |$$ |  $$ |$$\   $$ |
 \$$$$$$  |$$ |     \$$$$$$$ |$$$$$$$  | $$$$$$  |\$$$$$$  |
  \______/ \__|      \_______|\_______/  \______/  \______/ 
                     (\/) (°,,,,°) (\/)                                     
    "
    );

    logger::init(log::LevelFilter::Info);

    logger::info!("Starts the initialization sequence");

    logger::info!("---Global Descriptor Table and the kernel's Segments");
    gdt::init();
    logger::info!("---Interrupt Descriptor Table");
    idt::IDT.load();

    hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CrabOS::test_panic_handler(info)
}
