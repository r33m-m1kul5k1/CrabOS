//! native syscalls services

use log::info;

use crate::{syscalls::status, processes::get_process_info};

pub fn display_process_info(pid: usize) -> i64 {
    info!("process information: {:#?}", get_process_info(pid));
    status::SUCCESS
}