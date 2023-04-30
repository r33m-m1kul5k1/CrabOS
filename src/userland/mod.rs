//! this module includes userland processes
mod syscalls;
use syscalls::*;

pub fn user_main() -> ! {
    display_process_info(0).unwrap();
    loop {}
}
