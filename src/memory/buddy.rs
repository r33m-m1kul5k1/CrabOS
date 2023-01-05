//! This module implements the buddy system for allocating physical frames

use super::types::MemoryRegion;
use core::fmt;
use heapless::Vec;

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
