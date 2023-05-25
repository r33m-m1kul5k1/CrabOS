//! This module defines a minimal schduler that uses a stack to manage it's processes

use alloc::vec::Vec;
use log::debug;
use x86_64::structures::idt::InterruptStackFrame;

use super::objects::{Process, ProcessData, ProcessState};

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
    pub fn push_process(&mut self, process_code: u64) -> usize {
        let pid = self.next_pid();
        self.processes_stack.push(unsafe { Process::new(pid, process_code) });
        pid
    }

    /// Prepares the top process to start executing.
    /// 1. pauses the previous process & saves its state
    /// 2. activate the top process
    /// 3. returns a clone of the top process to run with
    /// 
    /// # Safety 
    /// 
    /// This function must be followd by and `Process::execute`
    pub fn get_process(&mut self, pid: usize) -> Result<Process, ()> {
        if pid > self.processes_stack.len() {
           return Err(())
        }

        self.processes_stack[pid].internal_data.state = ProcessState::Active;
        
        Ok(self.processes_stack[pid].clone())
    }

    /// Returns the next process id 
    pub fn next_pid(&self) -> usize {
        if self.processes_stack.len() == 0 {
            return 0
        }
        self.processes_stack.len()
    }

    /// Returns the process internal data.
    pub fn get_process_info(&self, pid: usize) -> Result<ProcessData, ()> {
        if pid > self.processes_stack.len() {
            return Err(())
         }
        Ok(self.processes_stack[pid].internal_data.clone())
    }

    /// Releases the process resources and remove it from the stack
    pub fn terminate_process(&mut self, pid: usize) -> Result<(), ()> {
        if pid > self.processes_stack.len() {
            return Err(())
        }
        for child_pid in (pid..=self.next_pid() -1).rev() {
            debug!("poping child: {:#x}", child_pid);
            self.processes_stack.last().ok_or(())?.release_resources();
            self.processes_stack.pop();
        }
        Ok(())
    }

    /// Pauses the a given process and saves it's state. Used when executing a new process.
    pub fn pause_process(&mut self, pid: usize, process_context: &InterruptStackFrame) {    
        self.processes_stack[pid].internal_data.state = ProcessState::Paused;
        self.processes_stack[pid].save_state(process_context);
    }
    
}

