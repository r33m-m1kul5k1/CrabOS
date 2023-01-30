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


pub const BIT_PRESENT: u16 = 1;
pub const BIT_WRITABLE: u16 = 1 << 1;
pub const BIT_USER: u16 = 1 << 2;
pub const BIT_WRITE_THROUGH: u16 = 1 << 3;
pub const BIT_NO_CACHE: u16 = 1 << 4;
pub const BIT_ACCESSED: u16 = 1 << 5;
pub const BIT_DIRTY: u16 = 1 << 6;
pub const BIT_HUGE: u16 = 1 << 7;
pub const BIT_GLOBAL: u16 = 1 << 8;

impl PTEntry {
    pub fn get_bit(&self, bit: u16) -> bool {
        (self.0 & (bit as u64)) != 0
    }

    pub fn set_opts(&mut self, options: u16) {
        let val = (self.0 >> 9) << 9;
        self.0 = val | options as u64;
    }

    pub fn set_bit(&mut self, bit: u16, v: bool) {
        if ((self.0 & (bit as u64)) != 0) != v {
            self.0 ^= bit as u64;
        }
    }

    pub fn set_phys_addr(&mut self, addr: PhysAddr) {
        let val = self.0 & ((1 << 9) - 1);
        self.0 = addr.addr() | val;
    }

    pub fn phys_addr(&self) -> PhysAddr {
        PhysAddr::new(self.0 & (((1 << 40) - 1) * FRAME_SIZE))
    }

    pub unsafe fn next_pt(&self) -> &'static mut PageTable {
        self.phys_addr().to_virt().unwrap().to_ref::<PageTable>()
    }
}

