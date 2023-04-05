//! This module controls a 4 level table structure.

use crate::memory::{KERNEL_ALLOCATOR, KERNEL_MAPPER};
use crate::memory::frame_distributer::FrameAllocator;

use super::types::FRAME_SIZE;
use bitflags::bitflags;
use core::{arch::asm, fmt};
use log::trace;

const ENTRY_ADDRESS_BITS: u64 = 0x000f_ffff_ffff_f000;

/// Creates a new mapping in the virtual address space of the calling process.
///
/// # Arguments
///
/// * `linear_addr` - starting linear address
/// * `length` - the new mapping's size in bytes
pub fn mmap(linear_addr: u64, length: usize) -> Result<(), ()> {

    for page_addr in (linear_addr..(linear_addr + length as u64)).step_by(FRAME_SIZE) {
        let physical_addr = KERNEL_ALLOCATOR.lock().allocate_frame().ok_or(())?;
        unsafe {
            KERNEL_MAPPER
                .lock()
                .map(
                    page_addr,
                    physical_addr,
                    &mut *KERNEL_ALLOCATOR.lock(),
                    EntryFlags::PRESENT | EntryFlags::WRITABLE,
                )?;
        };
        trace!("mapping {:x} to {:x}", page_addr, physical_addr);
    }
    Ok(())
}

/// A page table entry for 64 with PAE \
/// [tables structure format](https://wiki.osdev.org/File:64-bit_page_tables1.png)
#[repr(transparent)]
pub struct Entry {
    entry: u64,
}

impl Entry {
    /// Creates an unpresent entry
    #[inline]
    pub const fn new() -> Self {
        Entry { entry: 0 }
    }
    /// Returns the entry address
    #[inline]
    pub fn addr(&self) -> u64 {
        self.entry & ENTRY_ADDRESS_BITS
    }

    /// Sets entry flags
    #[inline]
    pub fn set_flags(&mut self, flags: EntryFlags) {
        self.entry |= flags.bits();
    }

    /// Returns the entry flags
    #[inline]
    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.entry)
    }

    /// Set entry address and flags
    ///
    /// # Arguments
    /// - `addr`, the address must be page aligned
    /// - `flags`, the entry flags
    #[inline]
    pub fn set_entry(&mut self, addr: u64, flags: EntryFlags) {
        assert!(addr == addr & ENTRY_ADDRESS_BITS);
        self.entry = addr | flags.bits();
    }

    /// Returns whether the entry is present or not
    #[inline]
    pub fn is_present(&self) -> bool {
        self.flags().contains(EntryFlags::PRESENT)
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Entry")
            .field("page frame base", &self.addr())
            .field("flags", &self.flags())
            .finish()
    }
}

#[repr(align(0x1000))]
#[repr(C)]
pub struct Table {
    pub entries: [Entry; 512],
}

bitflags! {
    pub struct EntryFlags: u64 {
        const PRESENT =             1;
        const WRITABLE =            1 << 1;
        const USER =                1 << 2;
        /// Updates both cache and the page frame
        const WRITE_THROUGH =       1 << 3;
        const DISABLE_CACHE =       1 << 4;
        /// If the entry was read during virtual address translation.
        const ACCESSED =            1 << 5;
        const DIRTY =               1 << 6;
        const PAGE_SIZE =           1 << 7;
        /// Cannot invalidate the TLB entry
        const GLOBAL =              1 << 8;

        const NO_EXECUTE =          1 << 63;
    }
}

pub fn get_cr3() -> u64 {
    let mut cr3: u64;
    unsafe {
        asm!(
            "mov {}, cr3",
            "mov cr3, rax",
            out(reg) cr3,
            options(nostack, preserves_flags),
        );
    }

    cr3
}
