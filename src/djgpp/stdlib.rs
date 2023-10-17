//! A subset from stdlib.h
//! 
//! We cannot use the libc crate because
//! the compilation target i686-unknown-none-gnu not recognized.

extern "C" {
    pub fn calloc(nmemb: usize, size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);
}
