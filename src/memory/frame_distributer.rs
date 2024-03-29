//! This modules defines the internal page frame allocation.

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

use crate::memory::types::{MemoryRegion, PAGE_SIZE};

/// A memory component which distributes page frames to the OS page frame allocators.\
/// It can distribute physical memory in chunks of 4Kib (frame).\
/// Or distribute a `FrameRange` of physical memory.\
/// This range size must power-of-two alignment and describe a continues memory.
pub struct FrameDistributer {
    /// Bootloader static memory map
    memory_map: &'static MemoryMap,
    /// Current frame index inside the usable memory regions
    current_frame: usize,
    /// Current usable `FrameRange`index   
    current_region: usize,
}

impl FrameDistributer {
    /// Create a new FrameDistributer from the passed bootloader's memory map.
    pub fn new(memory_map: &'static MemoryMap) -> Self {
        FrameDistributer {
            memory_map: memory_map,
            current_frame: 0,
            current_region: 0,
        }
    }

    /// Gets the next unused `FrameRange` see `FrameDistributer` documentation.
    pub fn get_region(&mut self) -> Option<MemoryRegion> {
        let unused_regions = self
            .memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable);

        let unused_regions = unused_regions
            .map(|r| r.range.start_addr()..r.range.end_addr())
            .map(|r| r.step_by(PAGE_SIZE as usize));

        log::trace!("The machine free regions are: ");
        for mut region in unused_regions.clone() {
            log::trace!(
                "region: {:#x}..{:#x}",
                region.next().unwrap(),
                region.last().unwrap()
            );
        }

        // this mapping converts an `Iterator<Iterator<"frames">>` to an Iterator of FrameRange
        let unused_regions = unused_regions.map(|region| {
            let mut region = MemoryRegion::new(
                region.clone().next().unwrap(),
                region.clone().last().unwrap(),
            );

            region.resize_region_range(self.next_frame_number());

            region.get_subregions()
        });

        // filter the invalid FrameRanges.
        let region = unused_regions
            .flat_map(|region| region)
            .filter(|region| {
                !region.is_empty() // is default
            })
            .nth(self.current_region)?;

        self.current_region += 1;

        Some(MemoryRegion::new(region.start_addr(), region.end_addr()))
    }

    /// Returns the unused frames iterator from the bootloader `memory_map`
    pub fn unused_frames(&self) -> impl Iterator<Item = u64> {
        let unused_regions = self
            .memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable);

        unused_regions
            .map(|r| r.range.start_addr()..r.range.end_addr())
            .flat_map(|r| r.step_by(PAGE_SIZE))
    }

    /// Returns the next free frame address
    fn next_frame_number(&self) -> u64 {
        self.unused_frames().nth(self.current_frame).unwrap()
    }
}

unsafe impl FrameAllocator for FrameDistributer {
    fn allocate_frame(&mut self) -> Option<u64> {
        let frame = self.unused_frames().nth(self.current_frame);
        self.current_frame += 1;

        frame
    }
}

/// A trait for types that can allocate a frame of memory.
///
/// # Safety
///
/// the frame alloctor must allocate unused memory frames
pub unsafe trait FrameAllocator {
    /// Allocate a frame and return it if possible.
    fn allocate_frame(&mut self) -> Option<u64>;
}
