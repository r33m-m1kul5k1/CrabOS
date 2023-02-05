//! The memory module goal is to
//! 1. allocate pages with vmalloc
//! 2. virutal memory mapping with mmap
//! 3. enables dynamic object use

pub mod buddy_system;
pub mod frame_distributer;
pub mod heap;
pub mod paging;
pub mod mapper;
pub mod types;

// use with caution |
//                  V

/// Converts an address to a const raw pointer
fn as_ptr<T>(address: u64) -> *const T {
    address as *const T 
}

/// Converts an adderss to a mutable raw pointer
fn as_mut_ptr<T>(address: u64) -> *mut T {
    as_ptr::<T>(address) as *mut T
}

/// Converts an address to a const reference
pub fn as_ref<'a, T>(address: u64) -> &'a T {
    unsafe { &*(address as *const T) }
}

/// Converts an address to a mutable reference
pub fn as_mut_ref<'a, T>(address: u64) -> &'a mut T {
    unsafe { &mut *as_mut_ptr::<T>(address)}
}
