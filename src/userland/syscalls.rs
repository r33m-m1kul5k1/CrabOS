//! This module define a comfortable inteface with the native syscall api of CrabOS.
use crate::syscalls::number::*;

#[macro_export]
macro_rules! syscall {
    ($number:expr) => {
        {
            let result: i64;
            core::arch::asm!(
                "int 0x80", 
                in("rax") $number,
                in("rdi") 0, 
                in("rsi") 0, 
                in("rdx") 0, 
                in("r8") 0,
                lateout("rax") result
            );
            result
        }
    };
    ($number:expr, $arg1:expr) => {
        {
            let result: i64;
            core::arch::asm!(
                "int 0x80", 
                in("rax") $number, 
                in("rdi") $arg1, 
                in("rsi") 0, 
                in("rdx") 0, 
                in("r8") 0,  
                lateout("rax") result
            );
            result
        }
    };
    ($number:expr, $arg1:expr, $arg2:expr) => {
        { 
            let result: i64;
            core::arch::asm!(
                "int 0x80", 
                in("rax") $number, 
                in("rdi") $arg1, 
                in("rsi") $arg2, 
                in("rdx") 0, 
                in("r8") 0, 
                lateout("rax") result
            );
            result
        }
    };
    ($number:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        {
            let result: i64;
            core::arch::asm!(
                "int 0x80", 
                in("rax") $number, 
                in("rdi") $arg1, 
                in("rsi") $arg2, 
                in("rdx") $arg3, 
                in("r8") 0, 
                lateout("rax") result
            );
            result
        }
    };
    ($number:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        {
            let result: i64;
            core::arch::asm!(
                "int 0x80", 
                in("rax") $number, 
                in("rdi") $arg1, 
                in("rsi") $arg2, 
                in("rdx") $arg3, 
                in("r8") $arg4, 
                lateout("rax") result
            );
            result
        }
    };
}

pub fn display_process_info(pid: usize) -> Result<(), ()> {
    let result = unsafe { syscall!(DISPLAY_PROCESS_INFO, pid) };
    
    if result >= 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn create(process_code: u64) -> Result<usize, ()> {
    let result = unsafe { syscall!(CREATE, process_code) };
    
    if result >= 0 {
        Ok(result as usize)
    } else {
        Err(())
    }
}

pub fn execute(pid: usize) {
    unsafe { syscall!(EXECUTE, pid) };
}

pub fn kill(pid: usize) {
    unsafe { syscall!(KILL, pid) };
}

pub fn get_pid() -> usize {
    unsafe { syscall!(GET_PID) as usize }
}