//! This module controls a 4 level table structure.
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page,
        PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use core::{fmt, arch::asm};
use bitflags::bitflags;

/// Creates a new mapping in the virtual address space of the calling process.
///
/// # Arguments
///
/// * `addr` - starting virtual address
/// * `length` - the new mapping's size in bytes
/// * `mapper` - maps pages to page frames.
/// * `frame_allocator` - allocate a frame from the physical address space
pub fn mmap(
    addr: VirtAddr,
    length: usize,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = Page::range_inclusive(
        Page::containing_address(addr),
        Page::containing_address(addr + length - 1u64),
    );

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    Ok(())
}


/// A page table entry for 64 with PAE \
/// [tables structure format](https://wiki.osdev.org/File:64-bit_page_tables1.png)
#[repr(transparent)]
pub struct Entry {
    entry: u64
}


impl Entry {

    /// Creates an unpresent entry
    #[inline]
    pub const fn new() -> Self {
        Entry { entry: 0 }
    }

    /// Returns the entry address (a page frame number)
    #[inline]
    pub fn addr(&self) -> u64 {
        self.entry & 0x000f_ffff_ffff_f000
    }

    /// Returns the entry flags
    #[inline]
    pub fn flags(&self)  -> EntryFlags {
        EntryFlags::from_bits_truncate(self.entry)
    }

    /// Set entry address and flags
    /// 
    /// # Arguments
    /// - `addr`, the address must be page aligned
    /// - `flags`, the entry flags
    #[inline]
    pub fn set_entry(&mut self, addr: u64, flags: EntryFlags) {
        assert!(addr == addr & 0x000f_ffff_ffff_f000);
        self.entry = addr | flags.bits();
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
   pub entries: [Entry; 512]
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


pub fn get_cr3()  -> u64 {
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