//! this modules defines the physical memory managers (frame distributer & buddy)

use bootloader::bootinfo::{FrameRange, MemoryMap, MemoryRegionType};
use log;
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};

const FRAME_SIZE: u64 = 4096;

/// the `FrameDistributer` is an Iterator that returns regions in power of 2
/// ## Fields
/// - `memory_map` - bootloader static memory map
/// - `next` - the index of the next free frame
pub struct FrameDistributer {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl FrameDistributer {
    pub fn new(memory_map: &'static MemoryMap) -> Self {
        FrameDistributer {
            memory_map: memory_map,
            next: 0,
        }
    }

    /// returns physical remaining frames range
    pub fn remaining_frames(&self) -> FrameRange {
        
        let remaining_frames = FrameRange {
            start_frame_number: self.unused_frames().nth(self.next).unwrap().start_address().as_u64() / FRAME_SIZE,
            end_frame_number: self.unused_frames().last().unwrap().start_address().as_u64() / FRAME_SIZE,
        };

        log::debug!("remaining frames are {:?}", remaining_frames);

        remaining_frames
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
}

unsafe impl FrameAllocator<Size4KiB> for FrameDistributer {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.unused_frames().nth(self.next);
        self.next += 1;

        frame
    }
}
