#![no_std] 
#![no_main] 

use core::panic::PanicInfo;

#[no_mangle] // don't give a random unique name for `_start` function
pub extern "C" fn _start() -> ! {
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}