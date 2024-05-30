//! Extern "C" subset of declarations from `stdlib.h`.
//!
//! We cannot use the `libc` crate
//! because the compilation target i686-unknown-none-gnu is not recognized.

#[allow(non_camel_case_types)]
pub type c_int = i32;

#[allow(non_camel_case_types)]
pub type c_short = i16;

#[allow(non_camel_case_types)]
pub type c_char = i8;

extern "C" {
    pub fn calloc(nmemb: usize, size: usize) -> *mut u8;
    pub fn free(ptr: *mut u8);

    pub fn getenv(_name: *const c_char) -> *const c_char;

    pub fn exit(c: c_int);
}
