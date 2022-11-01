#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(stmt_expr_attributes)]

mod interrupts;
mod logger;
mod serial;
mod vga_buffer;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use crate::vga_buffer::{Color, WRITER};
use interrupts::{gdt, idt};

entry_point!(kmain);

pub fn kmain(_boot_info: &'static BootInfo) -> ! {

    WRITER.lock().set_writer_theme(Color::LightRed, Color::Black);
    graphic_println!(r"
  $$$$$$\                     $$\        $$$$$$\   $$$$$$\  
 $$  __$$\                    $$ |      $$  __$$\ $$  __$$\ 
 $$ /  \__| $$$$$$\  $$$$$$\  $$$$$$$\  $$ /  $$ |$$ /  \__|
 $$ |      $$  __$$\ \____$$\ $$  __$$\ $$ |  $$ |\$$$$$$\  
 $$ |      $$ |  \__|$$$$$$$ |$$ |  $$ |$$ |  $$ | \____$$\ 
 $$ |  $$\ $$ |     $$  __$$ |$$ |  $$ |$$ |  $$ |$$\   $$ |
 \$$$$$$  |$$ |     \$$$$$$$ |$$$$$$$  | $$$$$$  |\$$$$$$  |
  \______/ \__|      \_______|\_______/  \______/  \______/ 
                     (\/) (°,,,,°) (\/)                                     
    ");

    logger::init(log::LevelFilter::Info);

    println!();
    logger::info!("Starts the initialization sequence");

    logger::info!("---Global Descriptor Table and the kernel's Segments");
    gdt::init();
    logger::info!("---Interrupt Descriptor Table");
    idt::IDT.load();
    
    #[allow(unconditional_recursion)] 
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