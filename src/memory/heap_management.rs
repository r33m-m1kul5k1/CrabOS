#![allow(dead_code)]

use crate::{exit_qemu, hlt_loop, QemuExitCode};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

// the heap will start after the first megabyte - https://wiki.osdev.org/Memory_Map_(x86)
const HEAP_BOTTOM: usize = 0x1_00000;
// one page table maps two Gigabyte - enough for one process
// max of 100 processes
const PAGE_TABLE_SIZE: usize = 512 * 64;
const PROCESS_OBJECT_SIZE: usize = 4096;
const MAX_PROCESSES: usize = 100;
const EXTRA_MULTIPLIER: usize = 4;
const HEAP_SIZE: usize =
    ((PAGE_TABLE_SIZE + PROCESS_OBJECT_SIZE) * MAX_PROCESSES) * EXTRA_MULTIPLIER;

#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> ! {
    log::error!("[Allocation Panic] {:?}", layout);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop()
}

#[global_allocator]
static ALLOCATOR: HeapManager = HeapManager::empty();

struct HeapManager {}

impl HeapManager {
    pub const fn empty() -> Self {
        HeapManager {}
    }

    pub unsafe fn init(&mut self, _heap_bottom: usize, _heap_size: usize) {}
}

unsafe impl GlobalAlloc for HeapManager {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}
