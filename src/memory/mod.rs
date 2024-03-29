//! The memory module goal is to
//! 1. allocate pages with vmalloc
//! 2. virutal memory mapping with mmap
//! 3. enables dynamic object use

use bootloader::BootInfo;
use lazy_static::lazy_static;
use log::{debug, info, trace};
use spin::Mutex;

use crate::memory::{
    buddy_system::manager::BuddyManager,
    frame_distributer::FrameDistributer,
    mapper::Mapper,
    paging::{get_cr3, EntryFlags, Table},
};

use self::types::VirtualMemoryRegion;

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

#[macro_export]
macro_rules! aligned_to_page_size {
    ($addr:expr) => {
        ($addr >> 12) << 12
    };
}

#[macro_export]
macro_rules! pages_iterator {
    ($first_page:expr, $size:expr) => {
        ($first_page..$first_page + (PAGE_SIZE * ($size)) as u64).step_by(PAGE_SIZE)
    };
}

/// Initialize frame distributer and a mapper to eventually initialize the kernel heap.
pub fn init(boot_info: &'static BootInfo) {
    debug!(
        "virtual memory offset: 0x{:x}",
        boot_info.physical_memory_offset
    );

    let mut frame_distributer = FrameDistributer::new(&boot_info.memory_map);
    info!("frame distributer initialized");

    KERNEL_MAPPER.lock().init(
        unsafe { as_mut_ref::<Table>(get_cr3() + boot_info.physical_memory_offset) },
        boot_info.physical_memory_offset,
    );
    info!("mapper initialized");

    heap::init(&mut frame_distributer);
    info!("kernel heap initialized");

    KERNEL_ALLOCATOR.lock().init(&mut frame_distributer);

    info!("finished initializing memory related structures");
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

/// Maps a memory region to a virtual memory region using the given flags
pub unsafe fn mmap(virtual_memory_region: VirtualMemoryRegion, flags: EntryFlags) -> Result<(), ()> {
    for (page, frame) in virtual_memory_region.pages_range.zip(virtual_memory_region.frames_range) {
        trace!("mapping page: {:#x} to frame: {:#x}", page, frame);
        kmap(page, frame, flags)?
    }

    Ok(())
}
/// Update pages access policy
///
/// # Arguments
///
/// - `start`, the staring page, must be align to page size
/// - `size`, number of pages to update their access policy
/// - `flags`, the new flags for the updated pages
/// 
/// # Safety
/// 
/// The given region and the flags not damage the kernel's virtual address space.
/// Otherwise it can lead to unpredictable behavior
pub unsafe fn update_pages_access_policy(virtual_memory_region: VirtualMemoryRegion, flags: EntryFlags) {
    for page in virtual_memory_region.pages_range {
        trace!("updating page {:#x}", page);
        KERNEL_MAPPER.lock().get_linear_address_entry(page).unwrap().set_flags(flags);
    }
}

/// Gets the start of the mapped physical memory
pub fn get_virutal_memory_base() -> u64 {
    KERNEL_MAPPER.lock().get_physical_memory_offset()
}

pub fn get_physical_addr(linear_addr: u64) -> Option<u64> {
    KERNEL_MAPPER.lock().linear_to_physical(linear_addr).ok()
}

pub fn get_page_frame(linear_addr: u64) -> Option<u64> {
    match KERNEL_MAPPER.lock().linear_to_physical(linear_addr) {
        Ok(linear_addr) => Some(aligned_to_page_size!(linear_addr)),
        Err(()) => None,
    }
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

#[macro_export]
macro_rules! code_addr {
    ($func_name:ident) => {
        $func_name as *const () as u64
    };
}