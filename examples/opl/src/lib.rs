//! Example Rust program that runs in MS-DOS
//! and plays some musical notes.
#![no_std]
#![no_main]
extern crate alloc;
use alloc::format;
use dos_x::djgpp::{dos::delay, stdio::puts, stdlib::exit};
use dos_x::adlib::{self, detect_adlib, reset_adlib};
use dos_x::sb::detect_sb;
use core::ffi::CStr;
use core::panic::PanicInfo;

use dos_x::key::wait_for_keypress;

#[allow(non_camel_case_types)]
type c_char = i8;

#[no_mangle]
fn dos_main() {

    // try to read command line arguments
    let args = dos_x::argv();

    let vgm_filename = if args.len() > 1 {
        // if there are any arguments, print them
        unsafe {
            puts(b"Command line arguments:\0".as_ptr() as *const c_char);
            for arg in args.iter().skip(1) {
                let str = core::ffi::CStr::from_ptr(*arg);
                let msg = format!(" - {}\0", str.to_string_lossy());
                puts(msg.as_ptr() as *const c_char);
            }
            Some(CStr::from_ptr(args[0]))
        }
    } else {
        None
    };

    unsafe {

        match detect_adlib() {
            0 => {
                puts(b"No Adlib sound card detected :(\0".as_ptr() as *const c_char);
                exit(0);
                core::hint::unreachable_unchecked()
            }
            _ => {
                puts(b"Adlib sound card detected!\0".as_ptr() as *const c_char);
            }
        };

        match detect_sb() {
            None => {
                puts(b"No Sound Blaster sound card detected :(\0".as_ptr() as *const c_char);
                exit(0);
                core::hint::unreachable_unchecked()
            }
            Some((addr, irq, dma)) => {
                let msg = format!("Sound Blaster sound card detected!\n addr: 0x{:x}\n irq: {}\n dma: {}\0", addr, irq, dma);
                puts(msg.as_ptr() as *const _);
            }
        };

        puts(b"Press Enter to continue\0".as_ptr() as *const c_char);

        busy_wait(5_000);
        wait_for_keypress(0x1c);

        app_loop();

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

fn app_loop() {

    // initialize
    reset_adlib();

    // set the instrument
    unsafe {
        // modulator multiple to 1
        adlib::write_command(0x20, 0x01);
        // modulator level
        adlib::write_command(0x40, 0x17);
        // modulator attack / release
        adlib::write_command(0x60, 0b1100_0100);
        // modulator sustain / release
        adlib::write_command(0x80, 0x77);
        // set carrier multiple to 1
        adlib::write_command(0x23, 0x01);
        // carrier level maximum volume (about 47db)
        adlib::write_command(0x43, 0x00);
        // carrier attack / release
        adlib::write_command(0x63, 0xf0);
        // carrier sustain / release
        adlib::write_command(0x83, 0x77);
    }

    unsafe {
        puts(b"Now playing...\0".as_ptr() as *const c_char);
    }

    let mut shutdown = false;
    let mut k = 0;
    while !shutdown {

        // play a note
        // (frequency depends on k)
        let freq: u16 = match k {
            0 => 0x2AE,
            1 => 0x181,
            2 => 0x1b0,
            3 => 0x1ca,
            4 => 0x202,
            5 => 0x241,
            6 => 0x287,
            7 => 0x2AE,
            _ => 0x2AE,
        };
        let [freq_lo, freq_hi] = freq.to_le_bytes();
        let octave: u8 = if k == 0 {
            3
        } else {
            4
        };
        unsafe {
            // set voice frequency LSB
            adlib::write_command(0xa0, freq_lo);
            // turn voice on, set octave, and freq MSB
            adlib::write_command(0xb0, (1 << 5) | (octave << 2) | freq_hi);
        }
        unsafe {
            delay(150);
            // release previous note
            adlib::write_command(0xb0, (octave << 2) | freq_hi);
        }

        if dos_x::key::get_keypress() == 0x1c {
            shutdown = true;
        }

        unsafe {
            delay(100);
        }

        k = (k + 1) & 0x07;

        if dos_x::key::get_keypress() == 0x1c {
            shutdown = true;
        }
    }

    // turn off voice
    unsafe {
        adlib::write_command(0xb0, 0x00);
    }
    reset_adlib();
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
