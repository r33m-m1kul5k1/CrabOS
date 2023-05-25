//! this module includes userland processes
mod syscalls;
use syscalls::*;

fn proc3() -> ! {
    display_process_info(get_pid()).unwrap();
    kill(2);
    loop {}
}

fn proc2() -> ! {
    display_process_info(get_pid()).unwrap();
    execute(
        create(proc3 as *const () as u64).unwrap()
        );
    loop {}
}

fn proc1() -> ! {
    display_process_info(get_pid()).unwrap();
    execute(
    create(proc2 as *const () as u64).unwrap()
    );
    exit()
}

pub fn user_main() -> ! {
    let child_pid = create(proc1 as *const () as u64).unwrap();
    display_process_info(child_pid).unwrap();
    execute(child_pid);
    display_process_info(get_pid()).unwrap();
    exit()
}

