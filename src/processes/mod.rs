//! this module includes:
//! 1. definisions of process and thread objects
//! 2. context switch - change the flow of execusion
//! 3. a scheduler

pub mod objects;
pub mod scheduler;

use lazy_static::lazy_static;
use spin::Mutex;
use scheduler::Scheduler;

lazy_static! {
    pub static ref KERNEL_SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::empty());
}

pub fn create_process(process_code: u64) {
    KERNEL_SCHEDULER.lock().push_process(process_code);
}

pub fn execute_process(pid: u64) { 
    KERNEL_SCHEDULER.lock().execute_process(pid).unwrap();
} 