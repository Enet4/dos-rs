//! Example Rust program that runs in MS-DOS
//! and plays some musical notes.
#![no_std]
#![no_main]
extern crate alloc;
use alloc::format;
use core::ffi::CStr;
use core::hint::unreachable_unchecked;
use core::panic::PanicInfo;
use dos_x::adlib::{self, detect_adlib, reset_adlib};
use dos_x::djgpp::{dos::delay, stdio::puts, stdlib::exit};
use dos_x::key::get_keypress;
use dos_x::println;
use dos_x::sb::detect_sb;
use opbinary::vgm::OplCommand;

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
            Some(CStr::from_ptr(args[1]))
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
                let msg = format!(
                    "Sound Blaster sound card detected!\n addr: 0x{:x}\n irq: {}\n dma: {}\0",
                    addr, irq, dma
                );
                puts(msg.as_ptr() as *const _);
            }
        };

        busy_wait(1_000);

        match vgm_filename {
            Some(filename) => run_player(filename),
            None => run_note_loop(),
        }

        println!("Bye");
    }
}

fn run_note_loop() {
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

    println!("Now playing...");

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
        let octave: u8 = if k == 0 { 3 } else { 4 };
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

fn run_player(filename: &CStr) {
    // initialize
    reset_adlib();

    println!("Opening file {}...", filename.to_string_lossy());

    let file_data = dos_x::fs::read(filename).unwrap_or_else(|e| {
        println!("Failed to read file: {}", e);
        unsafe {
            exit(-2);
            unreachable_unchecked();
        }
    });

    let vgm = match opbinary::vgm::Vgm::from_bytes(&file_data) {
        Ok(vgm) => {
            println!(
                "VGM v{:x}, {} samples",
                vgm.header.base_header.version, vgm.header.total_samples
            );

            if vgm.header.ym3812_clock == 0 && vgm.header.ymf262_clock == 0 {
                println!("Not an OPL music file");
                unsafe {
                    exit(-3);
                    unreachable_unchecked();
                }
            }
            if vgm.header.ymf262_clock != 0 {
                println!("YMF262 clock: {} Hz (OPL3)", vgm.header.ymf262_clock);
            }
            if vgm.header.ym3812_clock != 0 {
                println!("YM3812 clock: {} Hz (OPL2)", vgm.header.ym3812_clock);
            }

            vgm.into_opl_vgm()
        }
        Err(e) => {
            println!("Could not read VGM file: {}", e);
            unsafe {
                exit(-3);
                unreachable_unchecked();
            }
        }
    };

    println!("Now playing... (Hold Esc to stop)");

    fn samples_to_ms(samples: u32) -> u32 {
        samples * 10 / 441
    }

    for cmd in vgm.opl_commands {
        match cmd {
            OplCommand::Opl3 {
                port: 0,
                address,
                data,
            } => unsafe {
                adlib::write_command_l(address, data);
            },
            OplCommand::Opl3 {
                port: 1,
                address,
                data,
            } => unsafe {
                adlib::write_command_r(address, data);
            },
            OplCommand::Opl2 { address, data }
            | OplCommand::Opl3 {
                port: _,
                address,
                data,
            } => unsafe {
                adlib::write_command(address, data);
            },
            OplCommand::Wait { samples } => unsafe {
                delay(samples_to_ms(samples as u32));
                if get_keypress() == 1 {
                    break;
                }
            },
            OplCommand::SmallWait { n } => unsafe {
                delay(samples_to_ms(n as u32 + 1));
                if get_keypress() == 1 {
                    break;
                }
            },
            OplCommand::Wait735 => unsafe {
                delay(samples_to_ms(735));
                if get_keypress() == 1 {
                    break;
                }
            },
            OplCommand::Wait882 => unsafe {
                delay(samples_to_ms(882));
            },
        }
    }

    // turn off voice for every channel
    unsafe {
        for i in 0..9 {
            adlib::write_command(0xb0 | i, 0x00);
            delay(1);
            adlib::write_command_l(0xb0 | i, 0x00);
            delay(1);
            adlib::write_command_r(0xb0 | i, 0x00);
            delay(1);
        }
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
