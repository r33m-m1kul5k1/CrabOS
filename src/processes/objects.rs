//! this module defines thread and object structs

use core::arch::asm;
use crate::hlt_loop;

#[derive(Default)]
#[allow(unused)]
pub struct Thread {    
    ds:     u64,
    // registers
    rax:    u64,
    rbx:    u64,
    rcx:    u64,
    rdx:    u64,
    rsi:    u64,
    rdi:    u64,
    rbp:    u64,
    r8:     u64,
    r9:     u64,
    r10:    u64,
    r11:    u64,
    r12:    u64,
    r13:    u64,
    r14:    u64,
    r15:    u64,
    // ireq stack
    rip:    u64,
    cs:     u64,
    rflags: u64,
    rsp:    u64,
    ss:     u64,
}

impl Thread {
    /// Creates a new Thread object with the given stack, selectors and thread main
    /// 
    /// Note that the thread main should be the virtual address of the process main.
    pub fn new(thread_main: u64, cs: u16, ds: u16, ss: u16, rsp: u64) -> Self {
        Thread {
            rip: thread_main,
            cs: cs as u64,
            ds: ds as u64,
            ss: ss as u64,
            rsp,
            ..Default::default()
        }
    }
    /// Execute the current thread
    /// 
    /// # Saftey
    /// 
    /// The thread registers & selectors must be valid.
    #[inline]
    pub unsafe fn run(&self) -> ! {
        asm!(
        "mov rsp, {}",
        "pop rax; mov ds, ax",
        "pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp;\
         pop r8; pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15;",
        "iretq",
        in(reg) self);

        hlt_loop();
    }
}
