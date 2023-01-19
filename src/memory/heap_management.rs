
use core::{alloc::GlobalAlloc, ptr::null_mut};

use crate::{exit_qemu, hlt_loop, QemuExitCode};
use alloc::alloc::Layout;
use x86_64::{structures::paging::{Mapper, Size4KiB, FrameAllocator, mapper::MapToError}, VirtAddr};
use super::vmm::mmap;
use spin;

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
static ALLOCATOR: Locked<Dummy> = Locked::new(Dummy::empty());

/// Create a virtual address space for the heap (must be above the already mapped physical memory)
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>
) -> Result<(), MapToError<Size4KiB>> {

    mmap(VirtAddr::new(HEAP_BOTTOM), HEAP_SIZE, mapper, frame_allocator)?;
    
    Ok(())
}

pub struct Dummy {
}

impl Dummy {
    pub const fn empty() -> Self {
        Dummy {
        }
    }
}

unsafe impl GlobalAlloc for Locked<Dummy> {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}


/// A wrapper around spin::Mutex.
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


