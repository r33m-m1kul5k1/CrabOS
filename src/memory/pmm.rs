//! this modules defines the physical memory managers (frame distributer & buddy)

use bootloader::bootinfo::{FrameRange, MemoryMap, MemoryRegionType};
use log;

const FRAME_SIZE: u64 = 4096;
const INTEGER_SIZE: usize = 64;
const INVALID_REGION: FrameRange = FrameRange {
    start_frame_number: 0,
    end_frame_number: 0,
};
/// the `FrameDistributer` is an Iterator that returns regions in power of 2
/// ## Fields
/// - `memory_map` - bootloader static memory map
/// - `next` - the index of the next free region that is in power of 2
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

    /// given a region start and a region size, return a list of regions in the following format: 2^x
    fn get_subregions(
        region_start: u64,
        mut region_size: u64,
    ) -> [FrameRange; INTEGER_SIZE] {
        let mut subregions = [INVALID_REGION; INTEGER_SIZE];

        let mut offset_frame_number = region_start / FRAME_SIZE;

        for i in 0..INTEGER_SIZE {

            let subregion_size = (region_size & 1) << (i as u64);
            region_size = region_size >> 1;


            if subregion_size == 0 {
                continue;
            }

            subregions[i] = FrameRange {
                start_frame_number: offset_frame_number,
                end_frame_number: offset_frame_number + subregion_size,
            };

            offset_frame_number = subregions[i].end_frame_number;
        }

        log::trace!("subregions of region {:?} are: {:?}", region_start, subregions);
        subregions
    }
}

impl Iterator for FrameDistributer {
    type Item = FrameRange;

    /// gets the next unused region that is in size of 2^x.
    // NOTE: unused_region is a Map object meaning every time I use it it calls the maps and filter again
    fn next(&mut self) -> Option<Self::Item> {

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


        let unused_regions = unused_regions.map(|region| {
            let region_start = region.clone().next().unwrap();
            let region_size = region.clone().count() as u64;
            
            Self::get_subregions(region_start, region_size)
        });


        let region = unused_regions
            .flat_map(|region| region)
            .filter(|region| {
                region.start_addr() != INVALID_REGION.start_addr()
                    && region.end_addr() != INVALID_REGION.end_addr()
            })
            .nth(self.next);

        self.next += 1;

        region
    }
}
