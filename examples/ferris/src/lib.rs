//! Example Rust program that runs in MS-DOS.
//! 
//! 
#![no_std]
#![no_main]
extern crate alloc;
use alloc::vec::Vec;
use dos_x::djgpp::{stdio::puts, stdlib::exit};
use core::panic::PanicInfo;
use libm::sinf;

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
        let mut palette = ferris::ferris_color_palette();

        // apply the palette
        palette.set();
        // put the pixel data
        dos_x::vga::draw_buffer(&ferris);

        vsync();

        app_loop(&ferris);

        // fade out the screen
        for _ in 0..64 {
            for v in &mut palette.0 {
                *v = v.saturating_sub(1);
            }
            palette.set();
            vsync();
        }

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

#[inline(never)]
fn app_loop(ferris: &[u8]) {
    let mut shutdown = false;

    let gif_width = 320;
    let gif_height = 200;
    let mut k = 0;
    let mut angle = 0.;

    let mut video = [0; 320 * 200];

    while !shutdown {
        let s = sinf(angle * core::f32::consts::PI / 180.);
        k += 2;
        if k >= gif_width {
            k = 0;
        }
        angle += 3.;
        if angle > 360. {
            angle -= 360.;
        }
        let mut dest_ofs = 0;
        for y in 0..200 {
            for x in 0..320 {
                let osc = (s * 22.) as i32;
                let u = (x + k) % gif_width;
                let v = ((y as i32 + osc) as u32).clamp(0, gif_height - 1);
                let src_ofs = u + v * gif_width;
                video[dest_ofs] = ferris[src_ofs as usize];
                dest_ofs += 1;
            }
        }
        unsafe {
            vsync();
            dos_x::vga::draw_buffer(&video);
        }
        if dos_x::key::get_keypress() == 0x1c {
            shutdown = true;
        }
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
