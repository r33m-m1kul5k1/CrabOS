//! this modules defines the physical memory managers (frame distributer & buddy)

use bootloader::bootinfo::{FrameRange, MemoryMap, MemoryRegionType};
use core::ops::RangeInclusive;

use crate::{print, println};
const SIZE_4K: u32 = 4096;
const INTEGER_SIZE: usize = 32;
/// the `FrameDistributer` will distribute all the RAM frames to the different buddies
/// # Fields
/// - `unused_regions`, the list of lists of physical free frames.
pub struct FrameDistributer {
    memory_map: &'static MemoryMap,
    next: u32,
}

impl FrameDistributer {
    pub fn new(memory_map: &'static MemoryMap) -> Self {
        FrameDistributer {
            memory_map,
            next: 0,
        }
    }

    pub fn get_unused_region(&mut self) -> RangeInclusive<i32> {
        let unused_regions = self
            .memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable);

        /*
        converts the iterator of `MemoryRegion` to an iterator of iterators that describes frames
        */
        let unused_regions = unused_regions
            .map(|r| r.range.start_addr()..r.range.end_addr())
            .map(|r| r.step_by(SIZE_4K as usize));

        
        // TODO: add an Iterator for the distributed regions distributedRegions(unused_regions) (take(4))
        // let unsued_regions = unused_regions.flat_map(|region| {

        //         let first_address = region.clone().next().unwrap();
        //         let blocks = &mut Self::get_power2_blocks(region.clone().count() as u32);
               
               
        //         // blocks.iter().map(|block_size| {
        //         //     first_address..=(first_address + (SIZE_4K as u64) * (*block_size as u64))
        //         // })
        //         region
        //     }
            
            // );
        
        // consumes the iterator!
        for mut region in unused_regions {
            println!("{}", region.clone().count());
            let blocks = Self::get_power2_blocks(region.count() as u32);
            println!("{:?}", blocks);

            //println!("region {}..{}", region.next().unwrap(), region.last().unwrap());
        }

        //
        0..=0
    }

    /// given a number of frames, returns the blocks of frames that are at the power of two
    fn get_power2_blocks(mut frames: u32) -> [u32; INTEGER_SIZE] {
        let mut blocks = [0u32; INTEGER_SIZE];
        let base = 2u32;

        for i in 0..INTEGER_SIZE {
            blocks[i] = (frames & 1) * base.pow(i as u32);
            frames = frames >> 1;
        }

        blocks
    }
}
