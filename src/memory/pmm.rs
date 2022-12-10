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
        let mut frame = unused_frames.next();
        return frame;
    }

    /// gets an unused range of frames. note that this range must be in size of 4Kib * 2^x
    fn get_unused_range() -> PhysFrameRange<Size4KiB> {
        
        /*
        let mut it = PrevPeekable::new(unused_frames);
        let mut frame = it.next();
        let mut temp = frame;

        for frame in it {
            for frame in it
            if it.next().start_address() == (frame.start_address() + 4Kib) {

            }
        } */
    
        
    }

    fn get_unused_range_in_power2() -> PhysFrameRange<Size4KiB> {

        let mut it = PrevPeekable::new(unused_frames); // 2 directions iterator
        let mut range = get_unused_range();
        let mut start = range.start.start_address();
        let mut end = range.end.start_address();
        it = range.end;
        let num = end - start;
        while (num & (num - 1)) != 0 { // while end is not a power of 2
            end = it.prev();
        }
        start.range(self, end);
    }

}