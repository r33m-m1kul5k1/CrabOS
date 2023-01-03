//! this module defines types for memory components
use bootloader::bootinfo::FrameRange;

pub const FRAME_SIZE: u64 = 4096;
pub const INTEGER_SIZE: usize = 64;
pub const INVALID_FRAME_RANGE: FrameRange = FrameRange {
    start_frame_number: 0,
    end_frame_number: 0,
};

/// A struct representing a physical memory region
#[derive(PartialEq, Eq, Debug)]
pub struct MemoryRegion {
    /// The range of frames that belong to the region.
    pub range: FrameRange,
    /// The number of frames inside the region.
    pub size: usize,
}

impl MemoryRegion {
    /// Create a new FrameDistributer from the passed bootloader's memory map.
    pub fn new(start_frame_address: u64, end_frame_address: u64) -> Option<Self> {
        let region_range = FrameRange::new(start_frame_address, end_frame_address);

        Some(MemoryRegion {
            range: region_range,
            size: (region_range.end_frame_number - region_range.start_frame_number) as usize,
        })
    }
    /// Resize the memory region's range and it's size accordingly to the given new start address
    pub fn resize_region_range(&mut self, start_address: u64) {
        if start_address > self.range.end_addr() {
            *self = MemoryRegion::default();
        }

        if self.range.start_addr() < start_address && start_address < self.range.end_addr()
        {
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
}

impl core::default::Default for MemoryRegion {
    fn default() -> Self {
        MemoryRegion {
            range: FrameRange {
                start_frame_number: 0,
                end_frame_number: 0,
            },
            size: 0,
        }
    }
}
