#![feature(start, lang_items)]
#![no_std]
#![no_main]
use core::panic::PanicInfo;
use libc::{c_char, c_int};

extern "C" {
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn exit(c: c_int);
}

#[start]
#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const c_char) -> c_int {
    unsafe {
        puts(b"Rust says hello DOS!\0".as_ptr() as *const c_char);
    }
    0
}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    // exit using libc
    unsafe {
        exit(-1);
        core::hint::unreachable_unchecked()
    }
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
