//! Simple keyboard input module.

use djgpp::{pc::inportb, dpmi::__dpmi_yield};

pub fn wait_for_keypress(code: u8) {
    let mut c: u8 = 0;
    while c != code {
        unsafe {
            c = inportb(0x60);
            __dpmi_yield();
        }
    }
}

#[inline]
pub fn get_keypress() -> u8 {
    unsafe {
        inportb(0x60)
    }
}
