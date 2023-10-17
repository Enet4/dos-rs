//! Allocator

use core::alloc::{GlobalAlloc, Layout};

use crate::djgpp;

#[derive(Debug)]
struct AllocatorImpl;

unsafe impl Sync for AllocatorImpl {}

unsafe impl GlobalAlloc for AllocatorImpl {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        djgpp::stdlib::calloc(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        djgpp::stdlib::free(ptr)
    }
}

#[global_allocator]
static ALLOCATOR: AllocatorImpl = AllocatorImpl;
