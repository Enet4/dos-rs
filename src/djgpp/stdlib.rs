//! A subset from stdlib.h
//! 
//! We cannot use the libc crate because
//! the compilation target i686-unknown-none-gnu not recognized.

#[allow(non_camel_case_types)]
type c_int = i32;

extern "C" {
    pub fn calloc(nmemb: usize, size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);

    pub fn exit(c: c_int);
}
