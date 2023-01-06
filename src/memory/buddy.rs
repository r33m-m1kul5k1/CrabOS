//! This module implements the buddy system for allocating physical frames

use super::types::MemoryRegion;
use core::fmt;
use heapless::Vec;
use core::cmp;
use x86_64::PhysAddr;

/// [Buddy](https://wiki.osdev.org/Page_Frame_Allocation) is an allocation algorithm, running at O(log(n)) at worst case
pub struct Buddy<const MAX_ORDER: usize>
where
    [(); MAX_ORDER + 1]: Sized,
    [(); 1 << MAX_ORDER]: Sized,
{
    /// The physical region that buddy manages
    region: MemoryRegion,
    // The minimum size of a block the buddy system can allocate / deallocate.
    limit: u32,
    /// A vector of vectors describing the physical address space in different block sizes
    /// The capacity can be calculated by Vector's count * size of biggest vector
    free_blocks: Vec<Vec<u64, { 1 << MAX_ORDER }>, { MAX_ORDER + 1 }>,
}

impl<const MAX_ORDER: usize> Buddy<MAX_ORDER>
where
    [(); MAX_ORDER + 1]: Sized,
    [(); 1 << MAX_ORDER]: Sized,
{
    /// Initiate buddy with a region and a limit
    pub fn new(region: MemoryRegion, limit: u32) -> Self {
        /*
        this code is not working but we need to initalize the vec with some capacity
        let free_blocks = Vec::new();
        for _ in 0..(MAX_ORDER + 1) {
            free_blocks.push(Vec::with_capacity(4));
        }
         */
        Buddy {
            region: region,
            limit: limit,
            free_blocks: Vec::new(),
        }
    }

    #[allow(dead_code)]
    /// Return the biggest size of a block `Buddy` can allocate.
    fn block_max_size(&self) -> usize {
        // limit * 2 ^ (order_count -1)
        (self.limit as usize) << (MAX_ORDER as usize - 1)
    }
    //pub fn manages()


    // this func finds which memory level we need to use to serve a request for size bytes.
    pub fn req_size_to_level(&self, size: usize) -> Option<usize> {
        let max_size = self.block_max_size(); // finds the level of this allocator than can accommodate the required memory size.
        if size > max_size {
            None
        } else {
            let mut next_level = 1;
            while (max_size >> next_level) >= size {
                next_level += 1;
            }
            let req_level = cmp::min(next_level - 1, MAX_ORDER as usize);
            Some(req_level)
        }
    }

    // this func gets a block from the free list at this level or split a block above and return one of the splitted blocks.
    pub fn get_free_block(&mut self, level: usize) -> Option<u64> {
        self.free_blocks[level]
            .pop()
            .or_else(|| self.split_level(level))
    }

    // this func deals with the case of reaching the maximum level
    // there is no option to split the memory; this allocation is aborted
    // returns the other splitted block as we now have a block for this allocation
    pub fn split_level(&mut self, level: usize) -> Option<u64> {
        if level == 0 {
            None
        } else {
            self.get_free_block(level - 1).map(|block| {
                // first, we get a block from 1 level above us and split it.
                // second, we push the second of the splitted blocks to the current free list
                self.free_blocks[level].push(block * 2 + 1);
                block * 2
            })
        }
    }

    // this func is responsible for the allocation
    fn alloc(&mut self, size: usize, alignment: usize) -> Option<PhysAddr> {
        let size = cmp::max(size, alignment);
        self.req_size_to_level(size).and_then(|req_level| { // this line finds which level of this allocator can accommodate this amount of memory (if any)
            self.get_free_block(req_level).map(|block| {
                // get_free_block gives us the index of the block in the given level
                // so we need to find the size of each block in that level and multiply by the index
                // to get the offset of the memory that was allocated.
                let offset = block as u64 * (self.block_max_size() >> req_level as usize) as u64;
                // Add the base address of this buddy allocator's block and return
                PhysAddr::new(self.region.range.start_addr() + offset)
            })
        })
    }
}

impl<const MAX_ORDER: usize> fmt::Display for Buddy<MAX_ORDER>
where
    [(); MAX_ORDER + 1]: Sized,
    [(); 1 << MAX_ORDER]: Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "maximum order: {}",
            MAX_ORDER,
        )
    }
}
