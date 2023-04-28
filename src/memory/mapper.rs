//! This module defines a mapper object to map physical to virtual addresses.
use core::arch::asm;
use enum_iterator::{reverse_all, Sequence};
use log::debug;

use crate::memory::{as_addr, as_mut_ref};

use super::{
    frame_distributer::FrameAllocator,
    paging::{EntryFlags, Table, Entry},
};

pub struct Mapper<'a> {
    pml4_table: Option<&'a mut Table>,
    physical_memory_offset: u64,
}

impl<'a> Mapper<'a> {
    /// Creates an empty Mapper every call to map and translate linear address will do noting.
    pub const fn empty() -> Self {
        Mapper {
            pml4_table: None,
            physical_memory_offset: 0,
        }
    }
    /// Initialize a new mapper object given a pointer to the page table structures
    pub fn init(&mut self, pml4_table: &'a mut Table, physical_memory_offset: u64) {
        self.pml4_table = Some(pml4_table);
        self.physical_memory_offset = physical_memory_offset;
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
    /// 
    /// - `linear_addr`, the linear address 
    /// - `physical_addr`, the physical address
    /// - `frame_allocator`, frame allocator to allocate frames for new page tables.
    /// - `flags`, the linear address flags
    /// 
    /// # Safty
    ///
    /// The caller must specify an allocator that allocates only free frames
    pub unsafe fn map(
        &mut self,
        linear_addr: u64,
        physical_addr: u64,
        frame_allocator: &mut impl FrameAllocator,
        flags: EntryFlags,
    ) -> Result<(), ()> {
        let mut table_linear_address = as_addr::<Table>(self.pml4_table.as_ref().ok_or(())?);

        // Goes though pml4, pdp, pd, pt and initialize basic entries.
        for table_level in reverse_all::<PageTableLevel>() {
            let table = as_mut_ref::<Table>(table_linear_address);
            let entry = &mut table.entries[Mapper::entry_index(linear_addr, table_level)];

            if table_level == PageTableLevel::PageTable {
                entry.set_entry(physical_addr, flags);
            } else if !entry.is_present() {
                entry.set_entry(frame_allocator.allocate_frame().unwrap(), flags);
            } else {
                entry.set_flags(flags);
            }

            table_linear_address = entry.addr() + self.physical_memory_offset;
        }

        asm!("invlpg [{}]", in(reg) linear_addr, options(nostack, preserves_flags));
        Ok(())
    }

    /// Returns the page table entry for the following linear address
    pub fn get_linear_address_entry(&self, linear_addr: u64) -> Option<&mut Entry> {
        let mut table_linear_address = as_addr::<Table>(self.pml4_table.as_ref()?);
        let mut next_table: Option<&mut Entry> = None;
        // Goes though pml4, pdp, pd if the linear address offset doesn't exsist then return None.
        for table_level in reverse_all::<PageTableLevel>() {
            let table = unsafe { as_mut_ref::<Table>(table_linear_address) };
            next_table = Some(&mut table.entries[Mapper::entry_index(linear_addr, table_level)]);

            if !next_table.as_mut().unwrap().is_present() {
                return None
            }

            table_linear_address = next_table.as_mut().unwrap().addr() + self.physical_memory_offset;
        }

        next_table
    }
    /// Gets a physical address from a given linear address.
    pub fn linear_to_physical(&self, linear_addr: u64) -> Result<u64, ()> {
        let entry = self.get_linear_address_entry(linear_addr).ok_or(())?;
        debug!("{:x?}", entry);
        Ok(entry.addr())
    }

    /// Gets the mapper physical memory offset
    pub fn get_physical_memory_offset(&self) -> u64 {
        self.physical_memory_offset
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
