//! This module defines the syscall inteface and it's dispatcher
use core::arch::asm;

use log::{debug, error, info};
use x86_64::structures::idt::InterruptStackFrame;

use crate::{graphic_println, processes::objects::Registers};

macro_rules! wrap_syscall_handler {
    ($fn:ident => $wrapper:ident) => {
        #[naked]
        pub unsafe extern "C" fn $wrapper() {
            asm!(
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
                "mov rsi, rsp", // Arg #2: register list
                "mov rdi, rsp", // Arg #1: interrupt frame
                "add rdi, 9 * 8",
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
///  - ``
/// # Return Value
///
/// return the syscall result
extern "sysv64" fn syscall_handler(
    stack_frame: &mut InterruptStackFrame,
    registers: &mut Registers,
) {
    debug!("userland stack frame: {:#?}", stack_frame);
    registers.rax = dispatcher(
        registers.rax as u64,
        registers.rdi,
        registers.rsi,
        registers.rdx,
        registers.r8,
    );
}

#[allow(unused_variables)]
fn dispatcher(number: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> i64 {
    debug!(
        "syscall number {}\narg1 {}\narg2 {}\narg3 {}\narg4 {}",
        number, 
        arg1, 
        arg2, 
        arg3, 
        arg4
    );
    match number {
        number::PRINT => {
            
            graphic_println!(":)");
            0
        }
        number::LOG => {
            let message_ptr = arg1 as *mut u8;
            let len = arg2 as usize;
            let message = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(message_ptr, len)) };
            info!("{}", message);
            0
        }
        _ => {
            error!("unimplemented syscall");
            unimplemented!();
        }
    }
}

pub mod number {
    pub const PRINT: u64 = 0;
    pub const LOG: u64 = 1;
}