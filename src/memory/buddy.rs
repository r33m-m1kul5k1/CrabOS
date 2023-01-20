//! Page frame allocation allgorithm at O(log(n))

#![allow(unused)]
use alloc::vec::Vec;
use super::types::MemoryRegion;

const BUDDY_LIMIT: u32 = 0x1000;

struct Buddy {
    /// The physical region that buddy manages
    region: MemoryRegion,
    /// The maximum power of two that buddy can manage
    max_order: u32,
    /// A vector of vectors describing the physical address space in different block sizes
    free_blocks: Vec<Vec<u32>>,
}

impl Buddy {

    /// Creates buddy with a region and a limit of 4Kib
    /// 
    /// # Arguments
    /// 
    /// * `region` - a continues power-of-two aligned memory region
    /// 
    /// # Safety
    /// This function is unsafe because the caller must guarantee that the given
    /// buddy bounds are unused. This method must be called only once.
    pub unsafe fn new(region: MemoryRegion) -> Self {
        let max_order = region.size.count_zeros();
        Buddy {
            region,
            max_order,
            free_blocks: {
                let mut temp: Vec<Vec<u32>> = Vec::new(); 
                for _  in 0..max_order {
                    temp.push(Vec::<u32>::new());
                }
                temp

            }
        }

    }

}