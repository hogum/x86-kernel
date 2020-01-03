//!  Heap Allocator
//!  Contains the struct implementing the `GlobalAllocator` trait

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

/// Dummy ZST that implements `GlobalAlloc`
pub struct SimpleAlloc;

impl GlobalAlloc for SimpleAlloc {
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
