//! A simple module for video mode and VGA graphics in DOS.

use djgpp::dpmi::{__dpmi_int, __dpmi_regs};
use djgpp::go32::{_dosmemputw, dosmemput};
use djgpp::pc::{inportb, outportb};
use djgpp::sys::farptr::{_farpokeb, _farpokel};

/// The numerical address to the VGA buffer
pub(crate) const VGA_BUFFER_ADDR: u32 = 0xa0000;

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
    unsafe {
        let mut regs: __dpmi_regs = core::mem::zeroed();

        regs.x.ax = mode as u16;

        __dpmi_int(0x10, &mut regs);
    }
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
/// A video buffer of size 64_000 bytes
/// in VGA mode 13h is assumed.
#[inline]
pub unsafe fn put_pixel(x: u32, y: u32, c: u8) {
    if y >= 200 || x >= 320 {
        return;
    }

    let i = x + y * 320;

    unsafe {
        _farpokeb(djgpp::_dos_ds!(), VGA_BUFFER_ADDR + i, c);
    }
}

/// Draw a solid horizontal line at the given coordinates.
///
/// ### Safety
///
/// This function does not check whether the video mode is set correctly.
/// A video buffer of size 64_000 bytes
/// in VGA mode 13h is assumed.
#[inline]
pub unsafe fn draw_hline(x: i32, y: i32, length: u32, c: u8) {
    if y < 0 || y >= 200 {
        return;
    }
    let y = y as u32;
    // clamp x and length
    let x = x.max(0) as u32;
    let length = length.min(320 - x);

    let base = y * 320 + x;

    let cc = c as u16 | (c as u16) << 8;
    let cccc = cc as u32 | (cc as u32) << 16;

    // unroll into long far poke calls when possible
    let mut i = base;
    while i < base + length {
        if i & 3 == 0 && i + 3 < base + length {
            unsafe {
                _farpokel(djgpp::_dos_ds!(), VGA_BUFFER_ADDR + i as u32, cccc);
            }
            i += 4;
        } else {
            unsafe {
                _farpokeb(djgpp::_dos_ds!(), VGA_BUFFER_ADDR + i as u32, c);
            }
            i += 1;
        }
    }
}

/// Draw a solid vertical line at the given coordinates.
///
/// ### Safety
///
/// This function does not check whether the video mode is set correctly.
/// A video buffer of size 64_000 bytes
/// in VGA mode 13h is assumed.
#[inline]
pub unsafe fn draw_vline(x: i32, y: i32, length: u32, c: u8) {
    for j in 0..length {
        unsafe {
            put_pixel(x as u32, y as u32 + j, c);
        }
    }
}

/// Draw a solid rectangle at the given coordinates.
///
/// ### Safety
///
/// This function does not check whether the video mode is set correctly.
/// A video buffer of size 64_000 bytes
/// in VGA mode 13h is assumed.
pub unsafe fn draw_rect(x: i32, y: i32, width: u32, height: u32, c: u8) {
    for j in 0..height as i32 {
        unsafe {
            draw_hline(x, y + j, width, c);
        }
    }
}

/// Given a rectangular portion of pixel data
/// with the dimensions in `data_dim`,
/// copy a rectangular portion of it
/// defined by `origin` (x, y, width, height)
/// onto the display at the `target` (x, y) coordinates.
///
/// ### Safety
///
/// This function does not check whether the video mode is set correctly.
/// A video buffer of size 64_000 bytes
/// in VGA mode 13h is assumed.
#[inline]
pub unsafe fn blit_rect(
    data: &[u8],
    data_dim: (u32, u32),
    origin: (u32, u32, u32, u32),
    target: (i32, i32),
) {
    let (data_width, data_height) = data_dim;
    let (x, y, width, height) = origin;
    let (target_x, target_y) = target;

    let data = if data.len() > (data_width * data_height) as usize {
        &data[..(data_width * data_height) as usize]
    } else {
        data
    };

    let mut src = y * data_width + x;
    let mut target = target_y * 320 + target_x;
    for _ in 0..height {
        unsafe {
            // blit in contiguous data portions
            dosmemput(
                data.as_ptr().byte_offset(src as isize),
                width as usize,
                VGA_BUFFER_ADDR + target as u32,
            );
        }

        // next row
        src += data_width;
        target += 320;
    }
}

/// Draw the entirety of the given data buffer to the video buffer.
///
/// ### Safety
///
/// This function does not check whether the video mode is set correctly.
/// A video buffer of size 64_000 bytes is assumed.
#[inline]
pub unsafe fn draw_buffer(data: &[u8]) {
    let data = if data.len() > 320 * 200 {
        &data[..320 * 200]
    } else {
        data
    };
    unsafe {
        _dosmemputw(data.as_ptr(), data.len() / 2, VGA_BUFFER_ADDR);
    };
}

/// Synchronize the program with the vertical retrace
#[inline]
pub unsafe fn vsync() {
    unsafe {
        // wait until any previous retrace has ended
        loop {
            if (inportb(0x3DA) & 8) != 0 {
                break;
            }
        }

        /* wait until a new retrace has just begun */
        loop {
            if (inportb(0x3DA) & 8) == 0 {
                break;
            }
        }
    }
}

/// A thin abstraction over the VGA color palette.
///
/// The array within contains the 256 colors in RGB,
/// in standard layout (RGBRGBRGB...),
/// allocating 8 bits per channel
/// but only using 6 bits per channel.
///
/// If you prefer not to allocate,
/// see [`set_colors_with`]
/// or [`set_color_single`].
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
        unsafe {
            outportb(0x3c7, 0);
        }
        for p in &mut palette {
            *p = unsafe { inportb(0x3c9) };
        }
        Palette(palette)
    }

    /// Apply this palette in the system.
    pub fn set(&self) {
        // want to write
        unsafe {
            outportb(0x3c8, 0);
        }
        for p in &self.0 {
            unsafe {
                outportb(0x3c9, *p);
            }
        }
    }

    /// Apply a single color in this palette to the system's palette.
    pub fn set_single(&self, c: u8) {
        let i = c as usize * 3;
        let r = self.0[i];
        let g = self.0[i + 1];
        let b = self.0[i + 2];
        set_color_single(c, r, g, b);
    }
}

/// Set a single color in the VGA palette.
pub fn set_color_single(c: u8, r: u8, g: u8, b: u8) {
    unsafe {
        outportb(0x3c8, c);
        outportb(0x3c9, r);
        outportb(0x3c9, g);
        outportb(0x3c9, b);
    }
}

/// Reset the system's VGA palette
/// with the values coming from the given iterator.
///
/// The values from the iterator should be
/// consecutive 8-bit RGB values
/// (of 6-bit precision).
///
/// Iteration stops after 768 values.
pub fn set_colors_with(values: impl IntoIterator<Item = u8>) {
    // want to write
    unsafe {
        outportb(0x3c8, 0);
    }
    for p in values.into_iter().take(768) {
        unsafe {
            outportb(0x3c9, p);
        }
    }
}
