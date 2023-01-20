//! Defines a heap algorithm and initiate the heap virutal memory.
use crate::panic::{exit_qemu, hlt_loop, QemuExitCode};
use alloc::alloc::Layout;
use x86_64::{structures::paging::{Mapper, Size4KiB, FrameAllocator, mapper::MapToError}, VirtAddr};
use super::paging::mmap;
use linked_list_allocator::LockedHeap;

// Note that the heap must start at a page that is not already mapped.
const HEAP_BOTTOM: u64 = 0x_4444_4444_0000;

/// Heap size in bytes.
/// 
/// # Calculation
/// 
/// Long mode paging structure with one page table (2Mib) takes `0x1000 * 4`, [osdev](https://wiki.osdev.org/Page_Tables)\
/// Process object size 4Kib, multiply by 100 processes and an extra multiplier (4)
const HEAP_SIZE: usize = 0x80_0000;

/// The Heap order must be log2(HEAP_SIZE)
pub const HEAP_ORDER: usize = 23;

#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> ! {
    log::error!("[Allocation Panic] {:?}", layout);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Create a virtual address space for the heap (must be above the already mapped physical memory)
pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>
) -> Result<(), MapToError<Size4KiB>> {

    mmap(VirtAddr::new(HEAP_BOTTOM), HEAP_SIZE, mapper, frame_allocator)?;
    
    unsafe {
        ALLOCATOR.lock().init(HEAP_BOTTOM as usize, HEAP_SIZE);
    }

    Ok(())
}

