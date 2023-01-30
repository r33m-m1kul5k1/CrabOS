use alloc::boxed::Box;
use core::fmt::Display;

const VIRT_OFFSET: u64 = 0xC0000000;
pub const FRAME_SIZE: u64 = 0x1000;
type EmptyFrame = [u8; FRAME_SIZE as usize];

// page table entry struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PTEntry(u64); 

// page table struct
#[repr(C)]
pub struct PageTable {
    entries: [PTEntry; 512],
}
