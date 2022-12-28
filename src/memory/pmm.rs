//! this modules defines the physical memory managers (frame distributer & buddy)

use bootloader::bootinfo::{FrameRange, MemoryMap, MemoryRegionType};
use log;
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};

const FRAME_SIZE: u64 = 4096;
const INTEGER_SIZE: usize = 64;
const INVALID_REGION: FrameRange = FrameRange {
    start_frame_number: 0,
    end_frame_number: 0,
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

        #[cfg(test)]
        for mut region in unused_regions.clone() {
            log::trace!(
                "region: {:#x}..{:#x}",
                region.next().unwrap(),
                region.last().unwrap()
            );
        }

        let unused_regions = unused_regions.map(|region| {
            
            MemoryRegion::new(region.clone().next().unwrap() / FRAME_SIZE, region.clone().count()).unwrap().get_subregions()
        });

        let region = unused_regions
            .flat_map(|region| region)
            .filter(|region| {
                region.start_addr() != INVALID_REGION.start_addr()
                    && region.end_addr() != INVALID_REGION.end_addr()
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
        self.unused_frames().nth(self.current_frame).unwrap().start_address().as_u64() / FRAME_SIZE
    }
}

unsafe impl FrameAllocator<Size4KiB> for FrameDistributer {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.unused_frames().nth(self.current_frame);
        self.current_frame += 1;

        frame
    }
}

struct MemoryRegion {
    /// The range of frames that belong to the region.
    pub range: FrameRange,
    /// the number of frames inside the region.
    pub size: usize,
}

impl MemoryRegion {
    pub fn new(start_frame_number: u64, size: usize) -> Option<Self> {
        
        let mut region_range = FrameRange { start_frame_number: start_frame_number, end_frame_number: start_frame_number + size as u64 };
        let free_memory_address = 0; // FrameDistributer::get_free_memory_start();
        
        

        #[cfg(test)]
        log::trace!(
            "region: {:?}", region_range
        );

        Some(MemoryRegion {
            range: region_range,
            size: size,
        })
    }

    pub fn resize_region_range(&mut self, free_frame_number: u64) -> Result<Self, > {
        if free_frame_number > self.range.end_addr() {
            return None;
        }

        if self.range.start_addr() < free_frame_number && free_frame_number < self.range.end_addr() {
            region_range.start_frame_number = free_memory_address / FRAME_SIZE;
        }
    }

    /// given a region start and a region size, return a list of regions in the following format: 2^x
    pub fn get_subregions(&self) -> [FrameRange; INTEGER_SIZE] {
        let mut subregions = [INVALID_REGION; INTEGER_SIZE];
        let mut region_size = self.size;
        let mut offset_frame_number = self.range.start_frame_number;

        for i in 0..INTEGER_SIZE {
            let subregion_size = (region_size & 1) << (i as u64);
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

        #[cfg(test)]
        log::trace!(
            "subregions of region {:?} are: {:?}",
            self.range,
            subregions
        );

        subregions
    }
}
