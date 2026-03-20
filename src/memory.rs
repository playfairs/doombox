use x86_64::structures::paging::{
    FrameAllocator, Size4KiB,
};
use x86_64::structures::paging::PhysFrame;
use x86_64::PhysAddr;
use crate::println;

pub const PAGE_SIZE: usize = 4096;

pub struct DummyFrameAllocator;

impl DummyFrameAllocator {
    pub fn init() -> Self {
        DummyFrameAllocator
    }
}

unsafe impl FrameAllocator<Size4KiB> for DummyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        Some(PhysFrame::containing_address(
            PhysAddr::new(0x10000),
        ))
    }
}

pub fn init_memory() {
    println!("Initializing memory management...");
}
