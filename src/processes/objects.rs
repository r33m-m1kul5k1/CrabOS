//! this module defines thread and object structs

use core::arch::asm;
use log::info;

use crate::{
    interrupts::get_user_selectors,
    memory::{
        get_linear_addr, get_page_frame, kfree, kmalloc, mmap,
        paging::EntryFlags,
        types::{VirtualMemoryRegion, PAGE_SIZE},
        update_pages_access_policy,
    },
};
const PAGE_INDEX: u64 = 0xFFF;

#[derive(Default, Clone, Copy)]
pub struct Thread {
    context: Context,
}

#[derive(Default, Clone, Copy)]
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

#[derive(Default, Clone, Copy)]
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

#[derive(Clone)]
pub struct Process {
    pub internal_data: ProcessData,
    thread: Thread,
}

impl Process {
    /// Creates a new process object, allocate to the process a stack,
    /// and maps the process code to userland pages
    ///
    /// # Safety
    ///
    /// `process_code` must point to the process entry point or else unpredictable behavior may occur.  
    pub unsafe fn new(pid: usize, process_code: u64) -> Self {
        let (cs, ds) = get_user_selectors();
        let stack_top = kmalloc(PAGE_SIZE, PAGE_SIZE).unwrap();

        let code_page_frame = get_page_frame(process_code).unwrap();

        Process {
            internal_data: ProcessData {
                pid,
                code_region: VirtualMemoryRegion::new(get_linear_addr(code_page_frame), code_page_frame, 1),
                stack_region: VirtualMemoryRegion::new(get_linear_addr(stack_top), stack_top, 1),
                state: ProcessState::Waiting,
            },
            thread: Thread::new(
                get_linear_addr(code_page_frame) | (process_code & PAGE_INDEX),
                cs,
                ds,
                get_linear_addr(stack_top),
            ),
        }
    }

    /// Loads the process virtual address space and executes the process' main thread
    pub fn execute(&self) -> ! {
        unsafe {
            mmap(
                self.internal_data.stack_region.clone(),
                EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER,
            )
            .unwrap();
            mmap(
                self.internal_data.code_region.clone(),
                EntryFlags::PRESENT | EntryFlags::USER,
            )
            .unwrap();
        };

        info!("executing process: {}", self.internal_data.pid);
        unsafe { self.thread.run() }
    }

    /// Release the process' and thread' resources.
    pub fn release_resources(&self) {
        kfree(
            self.internal_data.stack_region.first_page(),
            self.internal_data.stack_region.size,
            PAGE_SIZE,
        );
        // unmaps the process virtual memory so that other processes wouldn't be able to access other processes data
        unsafe {
            update_pages_access_policy(self.internal_data.stack_region.clone(), EntryFlags::empty());
            update_pages_access_policy(self.internal_data.code_region.clone(), EntryFlags::empty());
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProcessData {
    pub pid: usize,
    pub code_region: VirtualMemoryRegion,
    pub stack_region: VirtualMemoryRegion,
    pub state: ProcessState,
}

#[derive(Clone, Copy, Debug)]
pub enum ProcessState {
    Active,
    Waiting,
}
