//! The memory module goal is to
//! 1. allocate pages with vmalloc
//! 2. virutal memory mapping with mmap
//! 3. enables dynamic object use

use bootloader::BootInfo;
use lazy_static::lazy_static;
use log::{info, warn};
use spin::Mutex;

use crate::memory::{
    buddy_system::manager::BuddyManager,
    frame_distributer::FrameDistributer,
    mapper::Mapper,
    paging::{get_cr3, Table},
};

use self::paging::EntryFlags;

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
    info!(
        "virtual memory base: 0x{:x}",
        boot_info.physical_memory_offset
    );

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

/// Maps a kernel page to a page frame
pub unsafe fn kmap(linear_addr: u64, physical_addr: u64, flags: EntryFlags) -> Result<(), ()> {
    unsafe {
        KERNEL_MAPPER.lock().map(
            linear_addr,
            physical_addr,
            &mut *KERNEL_ALLOCATOR.lock(),
            flags,
        )
    }
}

/// Gets the start of the mapped physical memory
pub fn get_virutal_memory_base() -> u64 {
    KERNEL_MAPPER.lock().get_physical_memory_offset()
}

pub fn get_physical_addr(linear_addr: u64) -> Option<u64> {
    let virtual_memory_base = get_virutal_memory_base();
    if linear_addr < virtual_memory_base {
        warn!(
            "Given linear address 0x{:x} is below virtual memory base at 0x{:x}",
            linear_addr, virtual_memory_base
        );
    }
    
    let physical_addr = KERNEL_MAPPER
        .lock()
        .linear_to_physical(linear_addr).ok();

    return physical_addr;
}

pub fn get_linear_addr(physical_addr: u64) -> u64 {
    physical_addr + get_virutal_memory_base()
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
