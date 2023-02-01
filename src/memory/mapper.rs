
//! This module defines a mapper object to map physical to virtual addresses.

use core::arch::asm;
pub struct Mapper {
    pml4_pointer: u64,
}

impl Mapper {

    /// Creates a new mapper object given a pointer to the page table structures
    pub fn new(pml4_pointer: u64) -> Self {
        Mapper { pml4_pointer }
    }

    /// Loads a new page table level 4 pointer to cr3 and flushes the TLB.
    /// 
    /// # Safty
    /// This method is unsafe because if the pml4 pointer is invalid the CPU will through an exception
    pub unsafe fn load_cr3(&self) {
        unsafe {
            
            asm!(
                "mov rax, cr3",
                "mov cr3, rax",
                "mov rax, {}",
                "mov cr3, rax",
                in(reg) self.pml4_pointer,
                options(nostack, preserves_flags),
            );
        }
    }
}