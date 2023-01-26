#![allow(unused)]
use alloc::vec::Vec;
use log::debug;
use x86_64::{PhysAddr, structures::paging::frame};
use crate::memory::frame_distributer::FrameDistributer;

use super::buddy::Buddy;

/// This manager manages multiple buddy algorithms
/// It divides the buddies to power-of-two memory regions
pub struct BuddyManager {
    buddies: Vec<Buddy>,
}

impl BuddyManager {

    /// Initialize the manager with buddies that manage the entire physical memory space
    pub fn new(frame_distributer: &mut FrameDistributer) -> Self {
        let mut buddies: Vec<Buddy> = Vec::new();
        
        while let Some(region) = frame_distributer.get_region() {
            buddies.push(unsafe {Buddy::new(region)});    
        }

        BuddyManager { buddies }
    }

    /// Allocates a given size of physical memory with the appropriate buddy
    pub fn allocate(&mut self, size: usize, alignment: usize) -> Option<PhysAddr> {
        
        for buddy in self.buddies.iter_mut() {
            if let Some(address) =  buddy.allocate(size, alignment) {
                return Some(address);
            }
        }

        None
    }

    /// Deallocates a physical block of memory with the appropriate buddy 
    pub fn deallocate(&mut self, address: PhysAddr, size: usize, alignment: usize) {
        
        for buddy in self.buddies.iter_mut() {
            if buddy.region.contains(address) {
                debug!("region: {:?}\naddr: {:?}", buddy.region, address);
                buddy.deallocate(address, size, alignment);
            }
        }
    }
}