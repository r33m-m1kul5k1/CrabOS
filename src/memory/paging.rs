//! This module controls a 4 level table structure.
use x86_64::{
    structures::paging::{OffsetPageTable, PageTable, Page, FrameAllocator, Size4KiB, mapper::MapToError, PageTableFlags, Mapper},
    VirtAddr,
};
use x86_64::registers::control::Cr3;


/// Initialize the page table mapper
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    
    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    log::debug!("The physical offset is: {:?}", physical_memory_offset);
    OffsetPageTable::new(&mut *page_table_ptr, physical_memory_offset)
}

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
    frame_allocator: &mut impl FrameAllocator<Size4KiB>
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = Page::range_inclusive(
        Page::containing_address(addr), 
        Page::containing_address(addr + length - 1u64)
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