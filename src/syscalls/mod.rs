//! This module defines the syscall inteface and it's dispatcher

mod services;

use core::arch::asm;
use log::{debug, error, trace};
use x86_64::structures::idt::InterruptStackFrame;

use crate::{processes::{objects::Registers, pause_process}, syscalls::services::*};


macro_rules! wrap_syscall_handler {
    ($fn:ident => $wrapper:ident) => {
        #[naked]
        pub unsafe extern "sysv64" fn $wrapper() {
            asm!(
                "mov rbp, rsp",
                "push r15",
                "push r14",
                "push r13",
                "push r12",
                "push r11",
                "push r10",
                "push r9",
                "push r8",
                "push rbp",
                "push rdi",
                "push rsi",
                "push rdx",
                "push rcx",
                "push rbx",
                "push rax",
                "mov rdi, rbp", // Arg #1: stack frame
                "mov rsi, rsp", // Arg #2: register list
                "call {}",
                "pop rax",
                "pop rbx",
                "pop rcx",
                "pop rdx",
                "pop rsi",
                "pop rdi",
                "pop rbp",
                "pop r8",
                "pop r9",
                "pop r10",
                "pop r11",
                "pop r12",
                "pop r13",
                "pop r14",
                "pop r15",
                "iretq",
                sym $fn,
                options(noreturn)
            );
        }
    };
}
wrap_syscall_handler!(syscall_handler => wrapped_syscall_handler);

/// Save the user process context and call the syscall dispatcher
///
/// # Arguments
///  - `registers`, the current userland registers
/// 
/// # Return Value
/// 
/// A 64 bit integer that if above or equal to zero the syscall was handled successfully,
/// otherwise while handling a failure occurred.
extern "sysv64" fn syscall_handler(
    stack_frame: &InterruptStackFrame,
    registers: &mut Registers,
) {
    trace!("stack frame: {:#x?}", stack_frame);
    let number = registers.rax as u64;
    let arg1 = registers.rdi;
    let arg2 = registers.rsi;
    let arg3 = registers.rdx;
    let arg4 = registers.r8;
    
    if number == number::EXECUTE {
        pause_process(arg1 as usize - 1, stack_frame);
        execute(arg1 as usize);
    } else {
        registers.rax = dispatcher(number, arg1, arg2, arg3, arg4);
    }

}

/// Handle all syscalls that require at max four arguments
#[allow(unused_variables)]
fn dispatcher(number: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> i64 {
    debug!(
        "syscall number {:#x}\narg1 {:#x}\narg2 {:#x}\narg3 {:#x}\narg4 {:#x}",
        number, 
        arg1, 
        arg2, 
        arg3, 
        arg4
    );
    match number {
        number::EXIT => {
            // TODO: add kill with the current pid
            status::SUCCESS
        }
        number::DISPLAY_PROCESS_INFO => {
            display_process_info(arg1 as usize)
        }
        number::CREATE => {
            create_process(arg1)
        }
        number::KILL => {
            kill(arg1 as usize)
        }
        _ => {
            error!("unimplemented syscall");
            unimplemented!();
        }
    }
}

/// Syscalls numbers
pub mod number {
    pub const EXIT: u64 = 0;
    pub const DISPLAY_PROCESS_INFO: u64 = 1;
    pub const CREATE: u64 = 2;
    pub const EXECUTE: u64 = 3;
    pub const KILL: u64 = 4;
}

/// Syscalls exit statuses
pub mod status {
    pub const SUCCESS: i64 = 0;
    pub const FAILURE: i64 = -1;
}