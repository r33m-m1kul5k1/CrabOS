//! This module defines a mapper object to map physical to virtual addresses.

#![allow(unused)]

use core::arch::asm;
use log::{info, debug};
use x86_64::registers::debug;

use super::{paging::{Table, EntryFlags}, frame_distributer::FrameAllocator};



pub struct Mapper<'a> {
    pml4_pointer: &'a mut Table,
}

impl<'a> Mapper<'a> {

    /// Creates a new mapper object given a pointer to the page table structures
    pub fn new(pml4_pointer: &'a mut Table) -> Self {
        Mapper { pml4_pointer }
    }

    /// Loads a new page table level 4 pointer to cr3 and flushes the TLB.
    /// 
    /// # Safty
    /// This method is unsafe because if the pml4 pointer is invalid the CPU will through an exception
    pub unsafe fn load_cr3(&self) {
        
        debug!("pml4 addr: {:p}", &self.pml4_pointer);
        asm!(
            "mov rax, cr3",
            "mov cr3, rax",
            "mov rax, {}",
            "mov cr3, rax",
            in(reg) &self.pml4_pointer,
            options(nostack, preserves_flags),
        );
        
    }

    /// Maps a linear 4KiB address to a physical one, and creates more paging tables if needed
    /// 
    /// # Safty
    /// 
    /// The caller must specify unmapped linear address
    pub unsafe fn map(
        &self,
        linear_addr: u64, 
        physical_addr: u64, 
        frame_allocator: &mut impl FrameAllocator, 
        flags: EntryFlags
    ) {

        debug!("pml4 first {:#x?}", self.pml4_pointer.entries[0]);

        asm!("invlpg [{}]", in(reg) linear_addr, options(nostack, preserves_flags));
    }
}


