//! DJGPP low level API
//! 
//! This no-std crate exposes the C API made available
//! when building programs targeting DOS systems via [DJGPP][]
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
pub mod dpmi;
pub mod dos;
pub mod go32;
pub mod pc;
pub mod malloc;
pub mod stdio;
pub mod stdlib;

pub mod sys;
