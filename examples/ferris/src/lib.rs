//! Example Rust program that runs in MS-DOS.
//! 
//! 
#![no_std]
#![no_main]
extern crate alloc;
use alloc::vec::Vec;
use dos_x::djgpp::{stdio::puts, stdlib::exit};
use core::panic::PanicInfo;

use dos_x::{key::wait_for_keypress, vga::vsync};

mod ferris;

#[allow(non_camel_case_types)]
type c_char = i8;

#[no_mangle]
fn dos_main() {
    unsafe {
        puts(b"Rust says hello DOS!\nPress Enter to continue\0".as_ptr() as *const c_char);

        busy_wait(100_000);
        wait_for_keypress(0x1c);

        dos_x::vga::set_video_mode_13h();

        // draw Ferris:

        // 1. grab pixel data
        let ferris = Vec::from(ferris::ferris_pixel_data());

        // 2. grab color palette
        let palette = ferris::ferris_color_palette();

        vsync();

        // apply the palette
        palette.set();
        // put the pixel data
        dos_x::vga::draw_buffer(&ferris);

        busy_wait(100_000);
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
}

/// wait a number of cycles
/// (use several thousands of cycles for a visible delay)
fn busy_wait(cycles: usize) {
    let mut dummy: u32 = 0;
    for i in 0..cycles {
        unsafe {
            core::ptr::write_volatile(core::hint::black_box(&mut dummy), i as u32);
        }
    }
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
