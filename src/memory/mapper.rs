//! This module defines a mapper object to map physical to virtual addresses.

#![allow(unused)]

use core::arch::asm;
use log::{debug, info};
use x86_64::registers::debug;

use crate::memory::{as_addr, as_mut_ref};

use super::{
    frame_distributer::FrameAllocator,
    paging::{Entry, EntryFlags, Table},
};

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
        flags: EntryFlags,
    ) {
        debug!("pml4 first {:#x?}", self.pml4_table.entries[0]);

        let mut table_address = as_addr::<Table>(self.pml4_table);

        // Goes though pml4, pdp, pd, pt and initialize a basic entries.
        for table_level in 4..1 {
            let table = as_mut_ref::<Table>(table_address);
            let next_table = &mut table.entries[Mapper::table_offset(linear_addr, table_level)];

            if table_level == 1 {
                next_table.set_entry(physical_addr, flags);
            }
            else if !next_table.is_present() {
                next_table.set_entry(
                    frame_allocator.allocate_frame().unwrap(),
                    EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::NO_EXECUTE,
                );
            }

            table_address = next_table.addr();
        }

        asm!("invlpg [{}]", in(reg) linear_addr, options(nostack, preserves_flags));
    }

    /// Gets a physical address from a given linear address.
    /// 
    pub fn linear_to_physical(&self, linear_addr: u64) -> Option<u64> {
        unimplemented!()
    }
    
    /// Gets the table index by a given table level and linear address
    ///
    /// # Levels
    ///
    /// - `page map level 4` => 4
    /// - `page directory pointer` => 3
    /// - `page directory` => 2
    /// - `page table` => 1
    fn table_offset(linear_addr: u64, level: usize) -> usize {
        (linear_addr & 0b1_1111_1111 << (9 * (level - 1) + 12)) as usize
    }
    
}
