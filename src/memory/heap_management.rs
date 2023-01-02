
use lazy_static::lazy_static;
use spin::Mutex;

use alloc::{alloc::{GlobalAlloc, Layout}, vec::{Vec, self}};
use core::ptr::null_mut;

use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

// the heap will start after the first megabyte - https://wiki.osdev.org/Memory_Map_(x86)
const HEAP_BOTTOM: usize = 0x1_00000;
// one page table maps two Gigabyte - enough for one process 
// max of 100 processes 
const PAGE_TABLE_SIZE: usize = 512 * 64;
const PROCESS_OBJECT_SIZE: usize = 4096; 
const MAX_PROCESSES: usize = 100;
const EXTRA_MULTIPLIER: usize = 4;
const HEAP_SIZE: usize = ((PAGE_TABLE_SIZE + PROCESS_OBJECT_SIZE) * MAX_PROCESSES) * EXTRA_MULTIPLIER; 

lazy_static! {
   pub static ref Test: Vec<u32> = {
    let mut t = alloc::vec![0, 1, 2, 3, 4];
    t.push(3);
    t
    };
}

#[global_allocator]
static ALLOCATOR: HeapManager = HeapManager::empty();

struct HeapManager {
    
}

impl HeapManager {
    
    pub const fn empty() -> Self {

        HeapManager {  }
    }

    pub unsafe fn init(&mut self, heap_bottom: usize, heap_size: usize) {

    }
}

unsafe impl GlobalAlloc for HeapManager {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}


pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_BOTTOM as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    // unsafe {
    //     ALLOCATOR.init();
    // }

    Ok(())
}


