//! this module implements the buddy system for allocating physical frames

use bootloader::bootinfo::FrameRange;
const FRAME_SIZE: u32 = 4096;

/// [Buddy](https://wiki.osdev.org/Page_Frame_Allocation) is an allocation algorithm, running at O(log(n)) at worst case
pub struct Buddy {
    /// the physical region that buddy manages
    region: FrameRange,
    /// the number of vectors representing different sizes
    order_count: u8,
    // the minimum size of a block the buddy system can allocate / deallocate.
    limit: u32,
    /// a vector of vectors describing the physical address space in different block sizes
    free_area: Vec<Vec<u32>>,
}

impl Buddy {
    /// Initiate buddy with a region and a limit
    pub fn init(region: FrameRange, limit: u32) {}

    /// Returns an empty buddy object to initiate
    pub const fn empty() -> Self {}

    /// Return the biggest size of a block `Buddy` can allocate.
    fn block_max_size(&self) -> usize {
        // limit * 2 ^ (order_count -1)
        (self.limit as usize) << (self.order_count as usize - 1)
    }
}
