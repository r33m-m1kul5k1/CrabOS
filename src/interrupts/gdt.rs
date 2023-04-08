//! This module constructs a Global Descriptor Table, and a Task State Segment

use lazy_static::lazy_static;
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{Segment, CS, DS};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;
pub const GENERAL_PROTECTION_FAULT_IST_INDEX: usize = 1;
pub const USER_STACK_INDEX: usize = 2;
pub const KERNEL_STACK_INDEX: usize = 0;
const PAGE_SIZE: usize = 4096;
const STACK_SIZE: usize = PAGE_SIZE * 4;

 
lazy_static! {
    /// # Task State Segment
    /// 
    /// A Table with stacks for interrupts and for different Privilege Levels
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = {
            // allocate a stack on the kernel's address space (.bss)
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            VirtAddr::from_ptr(unsafe { &STACK }) + STACK_SIZE
        };
        tss.interrupt_stack_table[GENERAL_PROTECTION_FAULT_IST_INDEX] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            VirtAddr::from_ptr(unsafe { &STACK }) + STACK_SIZE
        };

        tss.privilege_stack_table[KERNEL_STACK_INDEX] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            VirtAddr::from_ptr(unsafe { &STACK }) + STACK_SIZE
        };
        

        tss
    };
}

lazy_static! {
    /// # Global Descriptor Table
    /// 
    /// A Table with pointers to all the segments selectors
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        // will save the reserved stacks and the privileged stacks
        let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));
        // save all of these segments to switch between kernel mode and user mode segments
        let kernel_code = gdt.add_entry(Descriptor::kernel_code_segment());
        let kernel_data = gdt.add_entry(Descriptor::kernel_data_segment());
        let user_data = gdt.add_entry(Descriptor::user_data_segment());
        let user_code = gdt.add_entry(Descriptor::user_code_segment());

        (gdt, Selectors { tss, kernel_code, kernel_data, user_code, user_data })
     };
}

// `GlobalDescriptorTable` table's is private, that is why we use the Selectors struct 
pub struct Selectors {
    tss: SegmentSelector,
    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
}

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.kernel_code);
        DS::set_reg(GDT.1.kernel_data);
        load_tss(GDT.1.tss);
    }
}
