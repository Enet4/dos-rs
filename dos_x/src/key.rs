//! Simple keyboard input module.

use djgpp::{dpmi::__dpmi_yield, pc::{inportb, outportb}};

pub fn wait_for_keypress(code: u8) {
    let mut c: u8 = 0;
    while c != code {
        unsafe {
            c = inportb(0x60);
            __dpmi_yield();
        }
    }
}

/// Retrieve a single keypress from the keyboard buffer.
/// 
/// This function might not be as reliable as
/// [retrieving all keypresses](get_all_keypresses).
#[inline]
pub fn get_keypress() -> u8 {
    unsafe { inportb(0x60) }
}

/// Retrieve all keypresses in the keyboard buffer
/// and resets the "buffer full" status.
pub fn get_all_keypresses() -> smallvec::SmallVec<[u8; 4]> {
    let mut keys = smallvec::SmallVec::<[u8; 4]>::new();
    loop {
        let c = unsafe { inportb(0x60) };
        if c == 0 {
            break;
        }
        keys.push(c);
    }

    unsafe {
        let code = inportb(0x61);
        outportb(0x61, code | 0x80);
        outportb(0x61, code & !0x80);
    }

    keys
}
