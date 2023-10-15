//! GO32 functions (as declared in `go32.h`).

/* These lengths are in bytes, optimized for speed */

extern "C" {
    pub fn dosmemget(_offset: u32, _length: usize, _buffer: *mut u8);
    pub fn dosmemput(_buffer: *const u8, _length: usize, _offset: u32);
}

/* The lengths here are in TRANSFERS, not bytes! */

extern "C" {
    pub fn _dosmemgetb(_offset: u32, _xfers: usize, _buffer: *mut u8);
    pub fn _dosmemgetw(_offset: u32, _xfers: usize, _buffer: *mut u8);
    pub fn _dosmemgetl(_offset: u32, _xfers: usize, _buffer: *mut u8);
    pub fn _dosmemputb(_buffer: *const u8, _xfers: usize, _offset: u32);
    pub fn _dosmemputw(_buffer: *const u8, _xfers: usize, _offset: u32);
    pub fn _dosmemputl(_buffer: *const u8, _xfers: usize, _offset: u32);
}
