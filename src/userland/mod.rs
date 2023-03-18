//! this module includes userland processes
use log::info;


// the sign ! `never` means that the func never returns
pub fn dummy_process() -> ! {
    info!("hello from user land :)");
    loop {}
}
