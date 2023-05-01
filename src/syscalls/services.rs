//! native syscalls services

use log::info;

use crate::{syscalls::status, processes::{get_process_info, spawn_process, execute_process, kill_process}};

pub fn display_process_info(pid: usize) -> i64 {
    info!("process information: {:#x?}", get_process_info(pid));
    status::SUCCESS
}

pub fn create_process(process_code: u64) -> i64 {
    let pid = spawn_process(process_code) as i64;
    info!("spawned a process with pid {:x}", pid);
    
    if pid < 0 {
        status::FAILURE
    } else {
        pid
    }
}

pub fn execute(pid: usize) -> i64 {
    execute_process(pid);
    status::FAILURE
}

pub fn kill(pid: usize) -> i64 {
    kill_process(pid);
}