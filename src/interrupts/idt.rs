//! Initializing the Interrupt Descriptor Table with the valid interrupts 

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use super::gdt::{DOUBLE_FAULT_IST_INDEX, GENERAL_PROTECTION_FAULT_IST_INDEX};
use super::service_routines::{double_fault, general_protection_fault};


lazy_static! {
    /// # Interrupt Descriptor Table
    /// 
    /// A Table with descriptors about the interrupt service routines
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
            idt.general_protection_fault
                .set_handler_fn(general_protection_fault)
                .set_stack_index(GENERAL_PROTECTION_FAULT_IST_INDEX);
        }
        idt
    };
}
