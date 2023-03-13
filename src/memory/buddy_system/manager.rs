use super::{super::frame_distributer::FrameDistributer, buddy::Buddy};
use alloc::vec::Vec;
use lazy_static::lazy_static;
use log::debug;
use spin::Mutex;

lazy_static! {
    pub static ref KERNEL_ALLOCATOR: Mutex<BuddyManager> = Mutex::new(BuddyManager::empty());
}

/// This manager manages multiple buddy algorithms
/// It divides the buddies to power-of-two memory regions
pub struct BuddyManager {
    buddies: Vec<Buddy>,
}

impl BuddyManager {
    /// Creates an empty BuddyManager object.
    pub const fn empty() -> Self {
        BuddyManager {
            buddies: Vec::<Buddy>::new(),
        }
    }
    /// Initialize the manager with buddies that manage the entire physical memory space
    pub fn init(&mut self, frame_distributer: &mut FrameDistributer) {
        while let Some(region) = frame_distributer.get_region() {
            self.buddies.push(unsafe { Buddy::new(region) });
        }
    }

    /// Allocates a given size of physical memory with the appropriate buddy
    pub fn allocate(&mut self, size: usize, alignment: usize) -> Option<u64> {
        for buddy in self.buddies.iter_mut() {
            if let Some(address) = buddy.allocate(size, alignment) {
                return Some(address);
            }
        }

        None
    }

    /// Deallocates a physical block of memory with the appropriate buddy
    pub fn deallocate(&mut self, address: u64, size: usize, alignment: usize) {
        if let Some(buddy) = self
            .buddies
            .iter_mut()
            .find(|buddy| buddy.region.contains(address))
        {
            debug!("region: {:?}\naddr: {:?}", buddy.region, address);
            buddy.deallocate(address, size, alignment);
        } else {
            debug!("No buddy manages this memory :(");
        }
    }
}
