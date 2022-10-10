/* Initializing the Interrupt Descriptor Table with the valid interrupts */

use x86_64::structures::idt::{ InterruptDescriptorTable};
use lazy_static::lazy_static;

use crate::interrupts::service_routines::{ divide_error, double_fault, general_protection_fault };

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.divide_error.set_handler_fn(divide_error);
        idt.double_fault.set_handler_fn(double_fault);
        idt.general_protection_fault.set_handler_fn(general_protection_fault);
        idt
     };
}