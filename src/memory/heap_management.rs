#![allow(dead_code)]

use crate::{exit_qemu, hlt_loop, QemuExitCode};
use alloc::alloc::Layout;
use x86_64::{structures::paging::{Mapper, Size4KiB, FrameAllocator}, VirtAddr};
use super::{buddy::Buddy, vmm::mmap, types::MemoryRegion};

// Note that the heap must start at a page that is not already mapped.
const HEAP_BOTTOM: u64 = 0x_4444_4444_0000;
 
/// Heap size in bytes.
/// 
/// # Calculation
/// 
/// Long mode paging structure with one page table (2Mib) takes `0x1000 * 4`, [osdev](https://wiki.osdev.org/Page_Tables)\
/// Process object size 4Kib, multiply by 100 processes and an extra multiplier (4)
const HEAP_SIZE: usize = 0x80_0000;

/// log2(HEAP_SIZE)
const HEAP_ORDER: usize = 23;

#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> ! {
    log::error!("[Allocation Panic] {:?}", layout);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

#[global_allocator]
static ALLOCATOR: Locked<Buddy<HEAP_ORDER>> = Locked::new(Buddy::<HEAP_ORDER>::empty());

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>
) {
    mmap(VirtAddr::new(HEAP_BOTTOM), HEAP_SIZE, mapper, frame_allocator).unwrap();
    
    unsafe {
        ALLOCATOR.lock().init(
            MemoryRegion::new(HEAP_BOTTOM, HEAP_BOTTOM + HEAP_SIZE as u64),
             1);
    }
}

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}


