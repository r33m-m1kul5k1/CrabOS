//! The memory module goal is to
//! 1. allocate pages with vmalloc
//! 2. virutal memory mapping with mmap
//! 3. enables dynamic object use

use bootloader::BootInfo;
use log::info;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::memory::{
    buddy_system::manager::BuddyManager,
    frame_distributer::FrameDistributer,
    mapper::Mapper,
    paging::{get_cr3, Table},
};

pub mod buddy_system;
pub mod frame_distributer;
pub mod heap;
pub mod mapper;
pub mod paging;
pub mod types;


lazy_static! {
    pub static ref KERNEL_ALLOCATOR: Mutex<BuddyManager> = Mutex::new(BuddyManager::empty());
}

lazy_static! {
    pub static ref KERNEL_MAPPER: Mutex<Mapper<'static>> = Mutex::new(Mapper::empty());
}

/// Initialize frame distributer and a mapper to eventually initialize the kernel heap.
pub fn init(boot_info: &'static BootInfo) {
    
    let mut frame_distributer = FrameDistributer::new(&boot_info.memory_map);
    info!("frame distributer initialized");

    KERNEL_MAPPER.lock().init(
        unsafe { as_mut_ref::<Table>(get_cr3()) },
        boot_info.physical_memory_offset,
    );

    info!("mapper initialized");

    heap::init(&mut frame_distributer);
    info!("kernel heap initialized");

    KERNEL_ALLOCATOR.lock().init(&mut frame_distributer);

}

/// Allocate a kernel physical memory
pub fn kmalloc(size: usize, alignment: usize) -> Result<u64, ()> {
    KERNEL_ALLOCATOR.lock().allocate(size, alignment).ok_or(())
}

/// Frees a kernel physical memory
pub fn kfree(address: u64, size: usize, alignment: usize) {
    KERNEL_ALLOCATOR.lock().deallocate(address, size, alignment);
}

/// Converts an address to a const raw pointer
const fn as_ptr<T>(address: u64) -> *const T {
    address as *const T
}

/// Converts an adderss to a mutable raw pointer
///
/// /// # Safty
///
/// changing mutablility of a pointer neglects the immutability idea.
const unsafe fn as_mut_ptr<T>(address: u64) -> *mut T {
    as_ptr::<T>(address) as *mut T
}

/// Converts an address to a const reference
pub fn as_ref<'a, T>(address: u64) -> &'a T {
    unsafe { &*(address as *const T) }
}

/// Converts an address to a mutable reference
///
/// # Safty
///
/// changing an immutable pointer to a mutable reference neglects the immutability of the pointer
pub const unsafe fn as_mut_ref<'a, T>(address: u64) -> &'a mut T {
    &mut *as_mut_ptr::<T>(address)
}

pub fn as_addr<T>(object: &T) -> u64 {
    object as *const T as u64
}
