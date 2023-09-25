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
    graphic_println, hlt_loop,
    interrupts::{gdt, idt},
    log::{self, info, LevelFilter},
    memory::{self, frame_distributer::FrameDistributer, heap, paging},
    panic::PanicInfo, processes::{spawn_process, execute_process}, userland::user_main, code_addr,
};

entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {
    #[cfg(test)]
    test_main();
    
    log::init(LevelFilter::Info);
    display_logo();
    
    info!("CrabOS starts initialization sequence");
    gdt::init();
    idt::init();
    memory::init(boot_info);
    execute_process(spawn_process(code_addr!(user_main)));
    hlt_loop()
}

fn display_logo() {
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
