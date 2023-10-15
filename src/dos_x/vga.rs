//! A simple module for video mode and VGA graphics in DOS.

use crate::djgpp::dpmi::{__dpmi_regs, __dpmi_int};
use crate::djgpp::pc::{inportb, outportb};
use crate::djgpp::go32::{_dosmemputb, _dosmemputw};

const VGA_BUFFER_ADDR: u32 = 0xa0000;
const VGA_BUFFER_POINTER: *mut u8 = VGA_BUFFER_ADDR as *mut u8;

/// Set the video mode.
///
/// Example modes:
/// 
/// - 0x02 for 80x25 text mode
/// - 0x13 for 320x200 256-color mode
/// 
/// 
/// ### Safety
/// 
/// The caller must ensure that the video mode is valid.
#[inline]
pub unsafe fn set_video_mode(mode: u8) {
    let mut regs: __dpmi_regs = core::mem::zeroed();

    regs.x.ax = mode as u16;

    __dpmi_int(0x10, &mut regs);
}

/// Set the video mode to 13h: 320x200 256-color
#[inline]
pub fn set_video_mode_13h() {
    unsafe {
        set_video_mode(0x13);
    }
}

/// Put a single pixel value at the given coordinates.
/// 
/// ### Safety
/// 
/// This function does not check whether the video mode is set correctly.
/// A video buffer of size 64_000 bytes is assumed.
pub unsafe fn put_pixel(x: u32, y: u32, c: u8) {
    if y >= 200 || x >= 320 {
        return;
    }

    let i = x + y * 320;

    _dosmemputb(&c, 1, VGA_BUFFER_ADDR + i);
}

/// Draw the entirety of the given data buffer to the video buffer.
/// 
/// ### Safety
/// 
/// This function does not check whether the video mode is set correctly.
pub unsafe fn draw_buffer(data: &[u8]) {
    let data = if data.len() > 320 * 200 {
        &data[..320 * 200]
    } else {
        data
    };
    _dosmemputw(data.as_ptr(), data.len() / 2, VGA_BUFFER_ADDR);
}

/// A thin abstraction over the VGA color palette.
/// The array within contains the 256 colors in RGB,
/// in standard layout (RGBRGBRGB...),
/// allocating 8 bits per channel
/// but only using 6 bits per channel.
#[derive(Copy, Clone)]
pub struct Palette(pub [u8; 768]);

impl Palette {

    /// Create a new palette from a given array.
    #[inline]
    pub fn new(palette: [u8; 768]) -> Self {
        Self(palette)
    }

    /// Retrieve the palette currently defined in the system.
    pub fn get() -> Self {
        let mut palette = [0u8; 768];
        // want to read
        unsafe { outportb(0x3c7, 0); }
        for p in &mut palette {
            *p = unsafe { inportb(0x3c9) };
        }
        Palette(palette)
    }

    /// Apply this palette in the system.
    pub fn set(&self) {
        // want to write
        unsafe { outportb(0x3c8, 0); }
        for p in &self.0 {
            unsafe { outportb(0x3c9, *p); }
        }
    }
}
