//! An allocator which works on DOS.

use core::alloc::{GlobalAlloc, Layout};

#[derive(Debug)]
struct AllocatorImpl;

unsafe impl Sync for AllocatorImpl {}

unsafe impl GlobalAlloc for AllocatorImpl {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        djgpp::stdlib::calloc(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        djgpp::stdlib::free(ptr)
    }
}

#[global_allocator]
static ALLOCATOR: AllocatorImpl = AllocatorImpl;
