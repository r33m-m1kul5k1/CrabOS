//! this module includes userland processes
use core::arch::asm;

use log::info;

use crate::graphic_println;


// the sign ! `never` means that the func never returns
pub fn dummy_process() -> ! {
    info!("hello from user land :)");
    loop {}
}
pub fn logo_print() -> ! {
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
    loop {}
}

pub fn userland_shell() -> ! {
    unsafe { asm!("mov rax, 0xBEEF") };
    loop {}
}