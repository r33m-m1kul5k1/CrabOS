//! this modules defines the physical memory managers (frame distributer & buddy)

use bootloader::bootinfo::{FrameRange, MemoryMap, MemoryRegionType};

use crate::{println};
const FRAME_SIZE: u64 = 4096;
const INTEGER_SIZE: usize = 32;
/// the `FrameDistributer` will distribute all the RAM frames to the different buddies
/// # Fields
/// - `unused_regions`, the list of lists of physical free frames.
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
    fn get_region_memory_units(
        region_start: u64,
        mut region_size: u64,
    ) -> [FrameRange; INTEGER_SIZE] {
        let mut blocks = [FrameRange {
            start_frame_number: 0,
            end_frame_number: 0,
        }; INTEGER_SIZE];


        let mut offset_frame_number = region_start / FRAME_SIZE;

        for i in 0..INTEGER_SIZE {
            let block_size = (region_size & 1) << (i as u64);

            println!("block size: {}", block_size);
            if block_size == 0 {
                continue;
            }

            blocks[i] = FrameRange {
                start_frame_number: offset_frame_number,
                end_frame_number: offset_frame_number + block_size,
            };

            offset_frame_number = blocks[i].end_frame_number;
            region_size = region_size >> 1;
        }

        
        println!("{:?}\n", blocks);
        blocks
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


        for region in unused_regions.clone() {
            println!("{:?}", region);
        }

        let unused_regions = unused_regions.map(|region| {

            let region_start = region.clone().next().unwrap();
            let region_size = region.clone().count() as u64;

            // println!("region_start {:#x}", region_start);
            // println!("region_size {:#x}", region_size);
            Self::get_region_memory_units(region_start, region_size)
            
        });


        for region in unused_regions.clone() {
            
            // println!("{:?}\n", region);
            
        }

        let region = unused_regions.flat_map(|region| region).nth(self.next);

        self.next += 1;

        region
    }
}
