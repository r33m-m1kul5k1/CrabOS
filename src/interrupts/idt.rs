//! Initializing the Interrupt Descriptor Table with the valid interrupts

use lazy_static::lazy_static;
use log::info;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::syscalls::wrapped_syscall_handler;

use super::gdt::{
    DOUBLE_FAULT_IST_INDEX, GENERAL_PROTECTION_FAULT_IST_INDEX, PAGE_FAULT_IST_INDEX,
};
use super::service_routines::{double_fault, general_protection_fault, page_fault};

lazy_static! {
    /// # Interrupt Descriptor Table
    ///
    /// A Table with descriptors about the interrupt service routines
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
            idt.general_protection_fault
                .set_handler_fn(general_protection_fault)
                .set_stack_index(GENERAL_PROTECTION_FAULT_IST_INDEX as u16);
            idt.page_fault
                .set_handler_fn(page_fault)
                .set_stack_index(PAGE_FAULT_IST_INDEX as u16);
            idt[0x80]
                .set_handler_fn(core::mem::transmute(wrapped_syscall_handler as *mut fn()))
                .set_privilege_level(x86_64::PrivilegeLevel::Ring3);
        }
        idt
    };
}

pub fn init() {
    IDT.load();
    info!("IDT initialized");
}
