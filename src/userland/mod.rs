//! this module includes userland processes
mod syscalls;
use syscalls::*;

fn proc1() -> ! {
    display_process_info(1).unwrap();
    kill(1);
    loop {}
}

pub fn user_main() -> ! {
    // display_process_info(0).unwrap();
    let proc1_address = proc1 as *const () as u64;
    let child_pid = create(proc1_address).unwrap();
    display_process_info(child_pid).unwrap();
    execute(child_pid);
    loop {}
}

