#![allow(unused)]
use alloc::vec::Vec;
use super::buddy::Buddy;

/// This manager manages multiple buddy algorithms
/// It divides the buddies to power-of-two memory regions
struct BuddyManager {
    buddies: Vec<Buddy>,
}

impl BuddyManager {

    pub fn new() -> Self {
        unimplemented!();
    }

    // allocate and deallocate physical memory
}