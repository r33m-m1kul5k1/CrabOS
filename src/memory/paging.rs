//! This module controls a 4 level table structure.
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page,
        PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

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


// page table entry struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PTEntry(u64); 

// page table struct
#[repr(C)]
pub struct PageTable {
    entries: [PTEntry; 512],
}
