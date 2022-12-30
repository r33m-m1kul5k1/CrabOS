//! this modules defines the physical memory managers (frame distributer & buddy)

use bootloader::bootinfo::{MemoryMap, MemoryRegionType, FrameRange};

use crate::memory::types::{MemoryRegion, FRAME_SIZE};
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};

/// the `FrameDistributer` is an Iterator that returns regions in power of 2
/// ## Fields
/// - `memory_map` - bootloader static memory map
/// - `next` - the index of the next free frame
pub struct FrameDistributer {
    memory_map: &'static MemoryMap,
    current_frame: usize,
    current_region: usize,
}

impl FrameDistributer {
    pub fn new(memory_map: &'static MemoryMap) -> Self {
        FrameDistributer {
            memory_map: memory_map,
            current_frame: 0,
            current_region: 0,
        }
    }

    /// gets the next unused region that is in size of 2^x.
    pub fn get_region(&mut self) -> Option<FrameRange> {
        let unused_regions = self
            .memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable);

        /*
        converts the iterator of `MemoryRegion` to an iterator of iterators that describes frames
        */
        let unused_regions = unused_regions
            .map(|r| r.range.start_addr()..r.range.end_addr())
            .map(|r| r.step_by(FRAME_SIZE as usize));

        
        log::trace!("The machine free regions are: ");
        for mut region in unused_regions.clone() {
            log::trace!(
                "region: {:#x}..{:#x}",
                region.next().unwrap(),
                region.last().unwrap()
            );
        }

        let unused_regions = unused_regions.map(|region| {
            let mut region = MemoryRegion::new(
                region.clone().next().unwrap() / FRAME_SIZE,
                region.clone().count(),
            )
            .unwrap();

            region.resize_region_range(self.next_frame_number());

            region.get_subregions()
        });

        let region = unused_regions
            .flat_map(|region| region)
            .filter(|region| {
                !region.is_empty() // is default
            })
            .nth(self.current_region);

        self.current_region += 1;

        region
    }

    
    pub fn unused_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let unused_regions = self
            .memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable);

        unused_regions
            .map(|r| r.range.start_addr()..r.range.end_addr())
            .flat_map(|r| r.step_by(FRAME_SIZE as usize))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }

    fn next_frame_number(&self) -> u64 {
        self.unused_frames()
            .nth(self.current_frame)
            .unwrap()
            .start_address()
            .as_u64()
            / FRAME_SIZE
    }
}

unsafe impl FrameAllocator<Size4KiB> for FrameDistributer {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.unused_frames().nth(self.current_frame);
        self.current_frame += 1;

        frame
    }
}
