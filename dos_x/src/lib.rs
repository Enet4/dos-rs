//! A high level API providing access to DOS capabilities via DJGPP.
//! 
//! ## Using and building
//! 
//! Aside from including this in your project,
//! you also need to declare end user applications
//! as a static library:
//! 
//! ```toml
//! [lib]
//! crate-type = ["staticlib"]
//! ```
//! 
//! And bootstrap your program like this:
//!
//! ```rust
//! #![no_std]
//! #![no_main]
//!
//! #[no_mangle]
//! fn main() {
//!     // ...
//! }
//! ```
//! 
//! Note that for your programs to work with DJGPP,
//! they need to be linked together using the DJGPP GCC compiler.
//! Install pc-msdosdjgpp-gcc for the intended target architecture
//! (i386, i486, i586, or i686).
//! For the linking to be successful, you need to:
//!
//! 1. Build with the suitable target specification
//!    (see the JSON files for one of the supported architectures).
//!    For example:
//!    `cargo build --target i386-unknown-none-gnu.json`
//! 2. Grab the resulting static library archive file
//!    (e.g. `libmyapp.a`),
//!    extract all of its compiled objects using `ar`,
//!    and convert them all to COFF-GO32 using `elf2djgpp`.
//!    Then compile them all together into a new library archive file.
//! 3. Use the DJGPP compiler to create an executable:
//!    `i686-msdosdjgpp-gcc libmyapp.a -o myapp.exe`
//!
//! [DJGPP]: https://www.delorie.com/djgpp/

#![no_std]
#![feature(start)]
pub mod alloc;
pub mod vga;
pub mod key;
pub mod adlib;
pub mod sb;

// re-export
pub use djgpp;
use djgpp::stdlib::{c_int, c_char};

static mut ARGV: &'static [*const c_char] = &[];

pub fn argv() -> &'static [*const c_char] {
    unsafe {
        ARGV
    }
}

// This is the entry point for the DOS program via DJGPP
#[start]
#[no_mangle]
pub extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {

    // initialive arguments
    unsafe {
        ARGV = core::slice::from_raw_parts(argv as *const _, argc as usize);
    }

    extern "Rust" {
        fn dos_main();
    }

    unsafe {
        dos_main();
    }
    0
}
