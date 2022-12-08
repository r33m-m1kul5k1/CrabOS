//! this modules defines the physical memory managers (frame distributer & buddy)

use bootloader::BootInfo::{ self, MemoryMap};
use x86_64::structures::paging::{frame::PhysFrame, frame::PhysFrameRange, Size4KiB};
use x86_64::addr::PhysAddr;

const FRAME_SIZE: u32 = 4096;

/// the `FrameDistributer` will distribute all the RAM frames to the different buddies
/// # Fields
/// - `unused_frames`, the list of physical free frames.
/// - `next` points to the next free range
struct FrameDistributer {
    unused_frames: Iterator<Item = PhysFrame<Size4KiB>>,
    next: usize,
}



impl FrameDistributer {

    fn new(boot_info: BootInfo) -> None {

        /*
        `memory_map` is a list of memory regions 4Kib align,
        each `MemoryRegion` has a type enum called `MemoryRegionType`
        */
        let regions = boot_info.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        
        /*
        we use `map_flat` to remove the nesting from collections. 
        using `step_by` creates an iterator of frames inside the memory region.
        */
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(FRAME_SIZE));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)));
       

        FrameDistributer {
            unused_frames: frame_addresses,
            next: 0,
        }
    }

    /// gets an unused frame from the memory map
    fn get_unused_frame() -> PhysFrame<Size4KiB> {

    }

    /// gets an unused range of frames. note that this range must be in size of 4Kib * 2^x
    fn get_unused_range() -> PhysFrameRange<Size4KiB> {

    }

}