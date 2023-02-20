//! this module defines thread and object structs

pub struct Thread {
    /// program counter
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    /// stack pointer  
    pub rsp: u64,
    pub ss: u64,

    // registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8w: u64,
    pub r9w: u64,
    pub r10w: u64,
    pub r11w: u64,
    pub r12w: u64,
    pub r13w: u64,
    pub r14w: u64,
    pub r15w: u64,
    
}