//! malloc.h
//!
//! For other memory allocation functions,
//! see [`stdlib`](super::stdlib).

unsafe extern "C" {
    pub fn memalign(_align: usize, _amt: usize) -> *mut u8;
}
