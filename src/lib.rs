//! Example Rust program that runs in MS-DOS.
//! 
//! 
#![feature(start)]
#![no_std]
#![no_main]
use core::panic::PanicInfo;

use dos_x::key::wait_for_keypress;

mod djgpp;
mod dos_x;
mod ferris;

#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_char = i8;

// libc functions implemented by DJGPP
extern "C" {
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn exit(c: c_int);
}

#[start]
#[no_mangle]
pub extern "C" fn main(_argc: c_int, _argv: *const *const c_char) -> c_int {
    unsafe {
        puts(b"Rust says hello DOS!\nPress Enter to continue\0".as_ptr() as *const c_char);

        wait_for_keypress(0);
        wait_for_keypress(0x1c);

        dos_x::vga::set_video_mode_13h();

        // draw Ferris:

        // 1. grab pixel data
        let ferris = ferris::ferris_pixel_data();

        // 2. set up color palette
        ferris::ferris_color_palette().set();

        dos_x::vga::draw_buffer(ferris);
        
        wait_for_keypress(0);
        wait_for_keypress(0x1c);

        // set back to text mode
        dos_x::vga::set_video_mode(0x02);

        puts(b"< Bye >
 -----
    \\
     \\
       _~^~^~_
   \\) /  o o  \\ (/
     '_   -   _'
     / '-----' \\\0".as_ptr() as *const c_char);
    }

    0
}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    unsafe {
        // reset video mode
        dos_x::vga::set_video_mode(0x02);
        // exit using libc
        exit(-1);
        core::hint::unreachable_unchecked()
    }
}
