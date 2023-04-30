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