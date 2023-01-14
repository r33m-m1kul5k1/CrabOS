//! This module implements the buddy system for allocating physical frames

use super::{types::MemoryRegion, heap_management::Locked};
use core::{fmt, alloc::{Layout, GlobalAlloc}};
use heapless::Vec;
use x86_64::PhysAddr;
use core::cmp;

/// [Buddy](https://wiki.osdev.org/Page_Frame_Allocation) is an allocation algorithm, running at O(log(n)) at worst case
/// a region with size nKib,  nKib = (1 << MAX_ORDER) the index of the set bit is the MAX_ORDER
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

    /// Creates a buddy with constant default members
    pub const fn empty() -> Self {
        Buddy {
            region: MemoryRegion::empty(),
            limit: 0,
            free_blocks: Vec::new(),
        }
    }

    /// Initializing buddy with a region and a limit
    /// 
    /// # Arguments
    /// 
    /// * `region` - a continues power-of-two aligned memory region
    /// * `limit` - the minimum block size that can be allocated
    /// 
    /// # Safety
    /// This function is unsafe because the caller must guarantee that the given
    /// buddy bounds are unused. This method must be called only once.
    pub unsafe fn init(&mut self, region: MemoryRegion, limit: u32) {
        self.region = region;
        self.limit = limit;
        self.initialize_free_blocks()
    }

    /// Creates a buddy with initialized members
    /// 
    /// # Safety
    /// This function is unsafe because the caller must guarantee that the given
    /// buddy bounds are unused. This method must be called only once.
    pub unsafe fn new(region: MemoryRegion, limit: u32) -> Self {
        let mut new_buddy = Buddy::<MAX_ORDER>::empty(); 
        new_buddy.init(region, limit);
        new_buddy
    }

    // Initialize an empty vector of vectors for the free blocks vector
    fn initialize_free_blocks(&mut self) {

        for _ in 1..MAX_ORDER + 1 {
            let inner_vec: Vec<u64, { 1 << MAX_ORDER }> = Vec::new();
            self.free_blocks.push(inner_vec).unwrap();
        }

        self.free_blocks[0].push(0).unwrap();
    }

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

                log::debug!("splits level {}", order - 1);
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
                log::debug!("block's index: {}", block);
                // this calculation doesn't work
                let offset = block as u64 * (self.block_max_size() >> request_order as usize) as u64;
                // Add the base address of this buddy allocator's block and return
                PhysAddr::new(self.region.range.start_addr() + offset)
            })
        })
    }

    pub fn deallocate() {

    }
}


unsafe impl<const MAX_ORDER: usize> GlobalAlloc for Locked<Buddy<MAX_ORDER>> 
where
    [(); MAX_ORDER + 1]: Sized,
    [(); 1 << MAX_ORDER]: Sized,
{

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();
        allocator.allocate(layout.size(), layout.align()).unwrap().as_u64() as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
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
            "Buddy: [\nmaximum order: {}\nregion: {:?}\n]",
            MAX_ORDER,
            self.region
        )
    }
}

