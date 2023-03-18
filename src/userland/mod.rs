//! this module includes userland processes


// the sign ! `never` means that the func never returns
pub fn dummy_process() -> ! {
    // serial_println!("hello from user land :)");
    loop {}
}
