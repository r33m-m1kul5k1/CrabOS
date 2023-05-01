//! This module defines types for memory components
use core::{iter::StepBy, ops::Range, fmt};

use alloc::vec::Vec;
use bootloader::bootinfo::FrameRange;

use crate::pages_iterator;

pub const PAGE_SIZE: usize = 0x1000;
pub const INTEGER_SIZE: usize = 64;
pub const INVALID_FRAME_RANGE: FrameRange = FrameRange {
    start_frame_number: 0,
    end_frame_number: 0,
};

/// A struct representing a memory region (physical or virtual)
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MemoryRegion {
    /// The range of frames that belong to the region.
    pub range: FrameRange,
    /// The number of frames inside the region.
    pub size: usize,
}

impl MemoryRegion {
    pub const fn empty() -> Self {
        MemoryRegion {
            range: FrameRange {
                start_frame_number: 0,
                end_frame_number: 0,
            },
            size: 0,
        }
    }

    pub fn new(start_frame_address: u64, end_frame_address: u64) -> Self {
        let region_range = FrameRange::new(start_frame_address, end_frame_address);

        MemoryRegion {
            range: region_range,
            size: (region_range.end_frame_number - region_range.start_frame_number) as usize,
        }
    }
    /// Resize the memory region's range and it's size accordingly to the given new start address
    pub fn resize_region_range(&mut self, start_address: u64) {
        if start_address > self.range.end_addr() {
            *self = MemoryRegion::default();
        }

        if self.range.start_addr() < start_address && start_address < self.range.end_addr() {
            self.range = FrameRange::new(start_address, self.range.end_addr());
            self.size = (self.range.end_frame_number - self.range.start_frame_number) as usize;
        }
    }

    /// Given a region start and a region size, return a list of regions in the following format: 2^x
    pub fn get_subregions(&self) -> [FrameRange; INTEGER_SIZE] {
        let mut subregions = [INVALID_FRAME_RANGE; INTEGER_SIZE];

        if *self == MemoryRegion::default() {
            return [INVALID_FRAME_RANGE; INTEGER_SIZE];
        }

        let mut region_size = self.size;
        let mut offset_frame_number = self.range.start_frame_number;

        for i in 0..INTEGER_SIZE {
            // (current bit) * (2^i)
            let subregion_size = (region_size & 1) << (i as u64);
            // continue to the next bit
            region_size = region_size >> 1;

            if subregion_size == 0 {
                continue;
            }

            subregions[i] = FrameRange {
                start_frame_number: offset_frame_number,
                end_frame_number: offset_frame_number + subregion_size as u64,
            };

            offset_frame_number = subregions[i].end_frame_number;
        }

        log::trace!(
            "subregions of region {:?} are: {:?}",
            self.range,
            subregions
        );

        subregions
    }

    pub fn get_region_byte_size(&self) -> u32 {
        self.size as u32 * 0x1000u32
    }

    pub fn contains(&self, addr: u64) -> bool {
        // exclude the final page frame
        return self.range.start_addr() <= addr && addr <= self.range.end_addr() - PAGE_SIZE as u64;
    }
}

impl core::default::Default for MemoryRegion {
    fn default() -> Self {
        MemoryRegion::empty()
    }
}

/// This stuct descibe a continuous virtual memory region
#[derive(Clone)]
pub struct VirtualMemoryRegion {
    pub pages_range: StepBy<Range<u64>>,
    pub frames_range: StepBy<Range<u64>>,
    pub size: usize,
}

impl VirtualMemoryRegion {
    /// Creates a new virtual memory region and save its phyiscal frames
    pub fn new(first_page: u64, first_frame: u64, size: usize) -> Self {
        VirtualMemoryRegion {
            pages_range: pages_iterator!(first_page, size),
            frames_range: pages_iterator!(first_frame, size),
            size,
        }
    }

    pub fn first_page(&self) -> u64 {
        *self.pages_range.clone().collect::<Vec<u64>>().first().unwrap()
    }

    pub fn first_frame(&self) -> u64 {
        *self.frames_range.clone().collect::<Vec<u64>>().first().unwrap()
    }
}

impl fmt::Debug for VirtualMemoryRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Vritual Memory Region:")
            .field("first page", &self.first_page())
            .field("first frame", &self.first_frame())
            .field("size", &self.size)
            .finish()
    }
}

