//! this module defines thread and object structs

use core::arch::asm;
use log::debug;

use crate::{
    aligned_to_page_size,
    interrupts::get_user_selectors,
    memory::{
        get_linear_addr, get_page_frame, kmalloc, kmap, paging::EntryFlags,
        types::PAGE_SIZE, update_pages_access_policy,
    },
};
const PAGE_INDEX: u64 = 0xFFF;

#[derive(Default)]
pub struct Thread {
    context: Context,
}

#[derive(Default)]
#[allow(unused)]
pub struct Registers {
    pub rax: i64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

#[derive(Default)]
#[allow(unused)]
struct Context {
    ds: u64,
    // registers
    regisetrs: Registers,
    // ireq stack
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}
impl Thread {
    /// Creates a new Thread object with the given stack, selectors and thread main
    ///
    /// Note that the thread main should be the virtual address of the process main.
    pub fn new(thread_main: u64, cs: u16, ds: u16, rsp: u64) -> Self {
        Thread {
            context: Context {
                rip: thread_main,
                cs: cs as u64,
                ss: ds as u64,
                ds: ds as u64,
                rsp,
                ..Default::default()
            },
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
        "pop rax; mov ds, ax; mov es, ax; mov fs, ax; mov gs, ax",
        "pop rax; pop rbx; pop rcx; pop rdx; pop rsi; pop rdi; pop rbp;\
         pop r8; pop r9; pop r10; pop r11; pop r12; pop r13; pop r14; pop r15;",
        "iretq",
        in(reg) &self.context, options(noreturn));
    }
}

pub struct Process {
    pid: u64,
    thread: Thread,
}

impl Process {
    /// Creates a new process object, allocate to the process a stack,
    /// and maps the process code to userland pages
    ///
    /// # Safety
    ///
    /// `process_code` must point to the process entry point or else unpredictable behavior may occur.  
    pub unsafe fn new(pid: u64, process_code: u64, process_data: u64) -> Self {
        let (cs, ds) = get_user_selectors();
        let stack_top = kmalloc(PAGE_SIZE, PAGE_SIZE).unwrap();

        let code_page_frame = get_page_frame(process_code).unwrap();
        update_pages_access_policy(
            aligned_to_page_size!(process_data),
            2,
            EntryFlags::PRESENT | EntryFlags::USER,
        );

        unsafe {
            kmap(
                get_linear_addr(stack_top),
                stack_top,
                EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER,
            )
            .unwrap();
            kmap(
                get_linear_addr(code_page_frame),
                code_page_frame,
                EntryFlags::PRESENT | EntryFlags::USER,
            )
            .unwrap();
        };

        debug!(
            "process code page: {:#x} -> {:#x}",
            get_linear_addr(code_page_frame),
            get_page_frame(get_linear_addr(code_page_frame)).unwrap()
        );
        debug!(
            "process data page: {:#x} -> {:#x}",
            aligned_to_page_size!(process_data),
            get_page_frame(process_data).unwrap()
        );
        Process {
            pid,
            thread: Thread::new(
                get_linear_addr(code_page_frame) | (process_code & PAGE_INDEX),
                cs,
                ds,
                get_linear_addr(stack_top),
            ),
        }
    }

    /// Executes the process' main thread
    pub fn execute(&self) -> ! {
        debug!("executing process: {}", self.pid);
        unsafe { self.thread.run() }
    }
}
