//! This module defines a mapper object to map physical to virtual addresses.

#![allow(unused)]

use core::arch::asm;
use log::{info, debug};
use x86_64::registers::debug;

use super::{paging::{Table, EntryFlags, Entry}, frame_distributer::FrameAllocator};



pub struct Mapper<'a> {
    pml4_table: &'a mut Table,
}

impl<'a> Mapper<'a> {

    /// Creates a new mapper object given a pointer to the page table structures
    pub fn new(pml4_table: &'a mut Table) -> Self {
        Mapper { pml4_table }
    }

    /// Loads a new page table level 4 pointer to cr3 and flushes the TLB.
    /// 
    /// # Safty
    /// This method is unsafe because if the pml4 pointer is invalid the CPU will through an exception
    pub unsafe fn load_cr3(&self) {
        
        debug!("pml4 addr: {:p}", &self.pml4_table);
        asm!(
            "mov rax, cr3",
            "mov cr3, rax",
            "mov rax, {}",
            "mov cr3, rax",
            in(reg) &self.pml4_table,
            options(nostack, preserves_flags),
        );
        
    }

    /// Maps a linear 4KiB address to a physical one, and creates more paging tables if needed
    /// 
    /// # Safty
    /// 
    /// The caller must specify unmapped linear address
    pub unsafe fn map(
        &mut self,
        linear_addr: u64, 
        physical_addr: u64, 
        frame_allocator: &mut impl FrameAllocator, 
        flags: EntryFlags
    ) {
        debug!("pml4 first {:#x?}", self.pml4_table.entries[0]);
        
        // First 
        let pml4_index= Mapper::table_index(linear_addr, 4);
        let pte = &mut self.pml4_table.entries[pml4_index];
        
        if !pte.is_present() {
            
            pte.set_entry(
                frame_allocator.allocate_frame().unwrap(), 
            EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::NO_EXECUTE)
        }



        asm!("invlpg [{}]", in(reg) linear_addr, options(nostack, preserves_flags));
    }

    /// Gets the table index by a given table level and linear address
    /// 
    /// # Levels
    /// 
    /// - `page map level 4` => 4
    /// - `page directory pointer` => 3
    /// - `page directory` => 2
    /// - `page table` => 1
    fn table_index(linear_addr: u64, level: usize) -> usize {
        (linear_addr & 0b1_1111_1111 << (9 * (level - 1) + 12)) as usize
    }
}


