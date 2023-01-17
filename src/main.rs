#![no_std]
#![no_main]
#![allow(nonstandard_style)]
#![feature(abi_x86_interrupt)]
#![feature(stmt_expr_attributes)]
#![feature(custom_test_frameworks)]
#![test_runner(CrabOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;
use core::{panic::PanicInfo};
use CrabOS::{
    drivers::vga::{Color, WRITER},
    graphic_println, hlt_loop,
    interrupts::{gdt, idt},
    log::logger,
    memory::{vmm, pmm::FrameDistributer, heap_management::init_heap},
};

entry_point!(kmain);

pub fn kmain(boot_info: &'static BootInfo) -> ! {
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


    logger::init(log::LevelFilter::Debug);

    logger::info!("Starts the initialization sequence");

    logger::info!("Initializing the GDT");
    gdt::init();
    logger::info!("Initializing the IDT");
    idt::IDT.load();

    let mut frame_distributer = FrameDistributer::new(&boot_info.memory_map);
    
    logger::info!("Initializing paging");
    let mut mapper = unsafe {
        vmm::init(VirtAddr::new(boot_info.physical_memory_offset))
    };

    
    let _ =  [[0u64; 0x80_0000]; 23];

    logger::info!("Initializing heap");
    init_heap(&mut mapper, &mut frame_distributer).expect("heap initialization failed");

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
