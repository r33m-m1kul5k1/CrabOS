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
    /// Creates buddy with a region and a limit
    pub fn new(region: MemoryRegion, limit: u32) -> Self {

        Buddy {
            region,
            limit,
            free_blocks: Vec::new(),
        }
    }

    #[allow(dead_code)]
    /// Return the biggest size of a block `Buddy` can allocate.
    fn block_max_size(&self) -> usize {
        // limit * 2 ^ (max order)
        (self.limit as usize) << (MAX_ORDER as usize)
    }


    /// Finds the smallest block size that contains the request size bytes.
    /// Returns None if the request block size is bigger then buddy's region 
    pub fn get_order(&self, size: usize) -> Option<usize> {
        // finds the maximum block size which of this allocator.
        let max_size = self.block_max_size(); 
        if size > max_size {
            return None
        }

        let mut next_order = 1;
        // while the current order block size is >= request size check smaller block sizes
        while (max_size >> next_order) >= size {
            next_order += 1;
        }

        let request_order = cmp::min(next_order - 1, MAX_ORDER as usize);
        Some(request_order)
    
    }

    /// Gets a block from the free list at a given order or split a block above and return one of the splitted blocks.
    pub fn get_free_block(&mut self, order: usize) -> Option<u64> {
        self.free_blocks[order]
            .pop()
            .or_else(|| self.split_level(order))
    }

    /// This func deals with the case of splitting a block from a given order and returning the new block index.
    /// Reaching the maximum order, there is no option to split the memory; this allocation is aborted.
    pub fn split_level(&mut self, order: usize) -> Option<u64> {
        if order == 0 {
            None
        } else {
            self.get_free_block(order - 1).map(|block| {
                // first, we get a block from 1 level above us and split it.
                // second, we push the second of the splitted blocks to the current free list
                self.free_blocks[order].push(block * 2 + 1).unwrap();
                block * 2
            })
        }
    }

    /// Allocates a block given it's size and alignment
    pub fn allocate(&mut self, size: usize, alignment: usize) -> Option<PhysAddr> {
        let size = cmp::max(size, alignment);
         // this line finds which order of this allocator can accommodate this amount of memory (if any)
        self.get_order(size).and_then(|request_order| {
            self.get_free_block(request_order)
            .map(|block| {
                // to get the offset of the memory that was allocated
                // we multiply the block's size by it's index.
                let offset = block as u64 * (self.block_max_size() >> request_order as usize) as u64;
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
