//! this module includes userland processes
mod syscalls;
use syscalls::*;

use crate::code_addr;

fn proc3() -> ! {
    display_process_info(get_pid()).unwrap();
    kill(2);
    loop {}
}

fn proc2() -> ! {
    display_process_info(get_pid()).unwrap();
    execute(
        create(code_addr!(proc3)).unwrap()
        );
    loop {}
}

fn proc1() -> ! {
    display_process_info(get_pid()).unwrap();
    execute(
    create(code_addr!(proc2)).unwrap()
    );
    exit()
}

pub fn user_main() -> ! {
    let child_pid = create(code_addr!(proc1)).unwrap();
    display_process_info(child_pid).unwrap();
    execute(child_pid);
    display_process_info(get_pid()).unwrap();
    exit()
}

