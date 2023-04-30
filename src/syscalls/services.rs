//! native syscalls services

use log::info;

use crate::syscalls::status;

pub fn display_process_info(pid: usize) -> i64 {
    info!("pid: {}", pid);
    status::SUCCESS
}