//! Defines a heap algorithm and initiate the heap virutal memory.
use super::{frame_distributer::{FrameAllocator, FrameDistributer}, types::FRAME_SIZE};
use crate::{panic::{exit_qemu, hlt_loop, QemuExitCode}, memory::{paging::EntryFlags, KERNEL_MAPPER}};
use alloc::alloc::Layout;
use linked_list_allocator::LockedHeap;
use log::trace;

// Note that the heap must start at a page that is not already mapped.
const HEAP_BOTTOM: u64 = 0x10000f000;

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
pub fn init(frame_distributer: &mut FrameDistributer) {

    for page_addr in (HEAP_BOTTOM..(HEAP_BOTTOM + HEAP_SIZE as u64)).step_by(FRAME_SIZE) {
        let physical_addr = frame_distributer.allocate_frame().unwrap();
        unsafe {
            KERNEL_MAPPER
                .lock()
                .map(
                    page_addr,
                    physical_addr,
                    frame_distributer,
                    EntryFlags::PRESENT | EntryFlags::WRITABLE,
                )
                .unwrap()
        };
        trace!("mapping {:x} to {:x}", page_addr, physical_addr);
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_BOTTOM as usize, HEAP_SIZE);
    }
}
