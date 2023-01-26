#![allow(unused)]
use alloc::vec::Vec;
use x86_64::{PhysAddr, structures::paging::frame};
use crate::memory::frame_distributer::FrameDistributer;

use super::buddy::Buddy;

/// This manager manages multiple buddy algorithms
/// It divides the buddies to power-of-two memory regions
struct BuddyManager {
    buddies: Vec<Buddy>,
}

impl BuddyManager {

    pub fn new(mut frame_distributer: FrameDistributer) -> Self {
        let mut buddies: Vec<Buddy> = Vec::new();
        
        while let Some(region) = frame_distributer.get_region() {
            buddies.push(unsafe {Buddy::new(region)});    
        }

        BuddyManager { buddies }
    }

    pub fn allocate(&mut self, size: usize, alignment: usize) -> Option<PhysAddr> {
        unimplemented!()
    }

    pub fn deallocate(&mut self, address: PhysAddr, size: usize, alignment: usize) {

    }
}