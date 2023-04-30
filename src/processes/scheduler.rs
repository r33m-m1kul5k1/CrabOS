//! This module defines a minimal schduler that uses a stack to manage it's processes

use alloc::vec::Vec;

use super::objects::Process;

/// This object manages processes in CrabOS
/// The current process is poped out of the stack when it exits.
pub struct Scheduler {
    processes_stack: Vec<Process>,
}

impl Scheduler {
    /// Creates an empty scheduler
    pub const fn empty() -> Self {
       Scheduler { processes_stack: Vec::<Process>::new() }
    }

    /// Pushes a new process object to the scheduler's stack
    pub fn push_process(&mut self, process_code: u64) {
        let pid = self.current_pid() + 1;
        self.processes_stack.push(unsafe { Process::new(pid, process_code) });
        
    }

    /// Executes a process by a given pid
    pub fn execute_process(&self, pid: u64) -> Result<(), ()> {
        if pid > self.processes_stack.len() as u64 {
           return Err(())
        }
        self.processes_stack[pid as usize].execute();
    }
    
    /// Returns the current running process pid 
    pub fn current_pid(&self) -> u64 {
        if self.processes_stack.len() == 0 {
            return 0
        }
        (self.processes_stack.len() - 1) as u64
    }
}