//! This module defines a mapper object to map physical to virtual addresses.
use core::arch::asm;
use enum_iterator::{reverse_all, Sequence};
use log::debug;

use crate::memory::{as_addr, as_mut_ref};

use super::{
    frame_distributer::FrameAllocator,
    paging::{EntryFlags, Table},
};

pub struct Mapper<'a> {
    pml4_table: &'a mut Table,
    physical_memory_offset: u64,
}

impl<'a> Mapper<'a> {
    /// Creates a new mapper object given a pointer to the page table structures
    pub fn new(pml4_table: &'a mut Table, physical_memory_offset: u64) -> Self {
        Mapper {
            pml4_table,
            physical_memory_offset,
        }
    }

    /// Loads a new page table level 4 pointer to cr3 and flushes the TLB.
    ///
    /// # Safty
    /// This method is unsafe because if the pml4 pointer is invalid the CPU will through an exception
    pub unsafe fn load_cr3(&self) {
        debug!("pml4 addr: {:p}", &self.pml4_table);

        asm!("mov cr3, rax", in("rax") &self.pml4_table);
    }

    /// Maps a linear 4KiB aligned address to a physical one, and creates more paging tables if needed
    ///
    /// # Arguments
    /// - `physical_addr`, a linear address of the physical addres
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
        let mut table_linear_address = as_addr::<Table>(self.pml4_table);

        // Goes though pml4, pdp, pd, pt and initialize a basic entries.
        for table_level in reverse_all::<PageTableLevel>() {
            let table = as_mut_ref::<Table>(table_linear_address);
            let entry = &mut table.entries[Mapper::entry_index(linear_addr, table_level)];

            if table_level == PageTableLevel::PageTable {
                entry.set_entry(physical_addr, flags);
            } else if !entry.is_present() {
                entry.set_entry(
                    frame_allocator.allocate_frame().unwrap(),
                    EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::NO_EXECUTE,
                );
            }

            table_linear_address = entry.addr() + self.physical_memory_offset;
        }

        asm!("invlpg [{}]", in(reg) linear_addr, options(nostack, preserves_flags));
    }

    /// Gets a physical address from a given linear address.
    pub fn linear_to_physical(&self, linear_addr: u64) -> Option<u64> {
        let mut table_linear_address = as_addr::<Table>(self.pml4_table);

        // Goes though pml4, pdp, pd if the linear address offset doesn't exsist then return None.
        for table_level in reverse_all::<PageTableLevel>() {
            let table = unsafe { as_mut_ref::<Table>(table_linear_address) };
            let next_table = &mut table.entries[Mapper::entry_index(linear_addr, table_level)];

            if !next_table.is_present() {
                return None;
            }

            table_linear_address = next_table.addr() + self.physical_memory_offset;
        }

        Some(table_linear_address - self.physical_memory_offset)
    }

    /// Gets the entry index by a given table level and a linear address
    fn entry_index(linear_addr: u64, level: PageTableLevel) -> usize {
        linear_addr as usize >> (9 * (level as u64) + 12) & 0b1_1111_1111
    }
}

#[derive(Sequence, Clone, Copy, PartialEq)]
enum PageTableLevel {
    PageTable,
    PageDirectory,
    PageDirectoryPointerTable,
    PageMapLevelFour,
}
