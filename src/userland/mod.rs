//! this module includes userland processes
mod syscalls;
use syscalls::*;

fn proc2() -> ! {
    display_process_info(2).unwrap();
    kill(1);
    loop {}
}

fn proc1() -> ! {
    display_process_info(1).unwrap();
    let child_pid = create(proc2 as *const () as u64).unwrap();
    execute(child_pid);
    loop {}
}

pub fn user_main() -> ! {
    let child_pid = create(proc1 as *const () as u64).unwrap();
    display_process_info(child_pid).unwrap();
    execute(child_pid);
    display_process_info(0).unwrap();
    loop {}
}

