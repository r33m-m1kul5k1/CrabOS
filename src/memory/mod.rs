//! The memory module goal is to
//! 1. allocate pages with vmalloc
//! 2. virutal memory mapping with mmap
//! 3. enables dynamic object use

pub mod buddy_system;
pub mod frame_distributer;
pub mod heap;
pub mod paging;
pub mod types;
