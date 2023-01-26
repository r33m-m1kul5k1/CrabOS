//! Page frame allocation allgorithm at O(log(n))

use core::cmp;

use super::super::types::MemoryRegion;
use alloc::vec::Vec;
use log::debug;
use x86_64::PhysAddr;

const BUDDY_LIMIT: u32 = 0x1000;

pub struct Buddy {
    /// The physical region that buddy manages
    pub region: MemoryRegion,
    /// The maximum power of two that buddy can manage
    max_order: u32,
    /// A vector of vectors describing the physical address space in different block sizes
    free_blocks: Vec<Vec<u64>>,
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

        let max_order = region.size.trailing_zeros();
        debug!("max_order: {}", max_order);
        Buddy {
            region,
            max_order,
            free_blocks: {
                let mut temp: Vec<Vec<u64>> = Vec::new();
                for _ in 0..max_order {
                    temp.push(Vec::<u64>::new());
                }
                temp[0].push(0);
                temp
            },
        }
    }


    /// Return the biggest size of a block `Buddy` can allocate.
    fn block_max_size(&self) -> usize {
        // limit * 2 ^ (max order)
        (BUDDY_LIMIT << self.max_order) as usize
    }
    
    /// Gets a block from the free list at a given order or split a block above and return one of the splitted blocks.
    fn get_free_block(&mut self, order: usize) -> Option<u64> {
        self.free_blocks[order]
        .pop()
        .or_else(|| self.split_level(order))
    }
    
    /// Finds the smallest block size that contains the request size bytes.
    /// Returns None if the request block size is bigger then buddy's region 
    fn get_order(&self, size: usize) -> Option<usize> {
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

        // if the block is smaller then the minmum size then return the order of the minimum size
        let request_order = cmp::min(next_order - 1, self.max_order as usize);
        Some(request_order)
    
    }

    /// This func deals with the case of splitting a block from a given order and returning the new block index.
    /// Reaching the maximum order, there is no option to split the memory; this allocation is aborted.
    fn split_level(&mut self, order: usize) -> Option<u64> {

        if order == 0 {
            None
        } else {
            
            self.get_free_block(order - 1).map(|block| {
                
                debug!("splits level {}", order - 1);
                // first, we get a block from 1 level above us and split it.
                // second, we push the second of the splitted blocks to the current free list
                self.free_blocks[order].push(block * 2 + 1);
                block * 2
            })
        }
    }

    /// Merges two buddies if they exists
    pub fn merge_buddies(&mut self, order: usize, block: u64) {
        
        debug!("free_blocks: {:?}", self.free_blocks);
        let buddy_block = block ^ 1;

        // don't merge if the block's buddy is used or the block is the parent block (0)
        if !self.free_blocks[order].contains(&buddy_block) {
            return;
        }


        self.free_blocks[order].pop();
        self.free_blocks[order].retain(|value| *value != buddy_block);
        self.free_blocks[order - 1].push(block / 2);

        self.merge_buddies(order - 1, block / 2);

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
                debug!("block's index: {}", block);
                debug!("free blocks: {:?}", self.free_blocks);
                // index * order size
                let offset = block as u64 * (self.block_max_size() >> request_order as usize) as u64;
                // Add the base address of this buddy allocator's block and return
                PhysAddr::new(self.region.range.start_addr() + offset)
            })
        })
    }

    /// Deallocate a block by pushing it back to it's vector and merging it if needed
    /// 
    /// # Arguments
    /// * `address` - the addres of the block
    /// * `size` - block's size
    /// * `alignment` - a **power of two** block's alignment
    pub fn deallocate(&mut self, address: PhysAddr, size: usize, alignment: usize) {
        let size = cmp::max(size, alignment);
        
        let order = self.get_order(size).unwrap();
        let block = (address - self.region.range.start_addr()).as_u64() / size as u64;

        self.free_blocks[order].push(block);
        self.merge_buddies(order, block);
    }
}
