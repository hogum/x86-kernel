//!  Heap Allocator
//!  Contains the struct implementing the `GlobalAllocator` trait

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

/// HEAP memory starting address
pub const HEAP_START: usize = 0x_4444_4444_0000;
/// 100 KB heap size
pub const HEAP_SIZE: usize = 100 * 1024;

/// Dummy ZST that implements `GlobalAlloc`
pub struct SimpleAlloc;

unsafe impl GlobalAlloc for SimpleAlloc {
    /// Allocates heap memory
    /// Returns a raw pointer to the first byte of the
    /// allocated memory block
    ///
    /// Null pointer signals an allocation error
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }
    /// Frees an allocated memory block
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // allocater never returns memory
        // call to `dealloc` shouldn't occure
        panic!("Oops! Not sane to call `dealloc`")
    }
}

/// Maps Virtual heap memory region to Physical memory
pub fn map_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64; // Inclusive bound on last byte address
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        mapper.map_to(page, frame, flags, frame_allocator)?.flush();
    }
    unsafe {
        super::ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
    Ok(())
}
