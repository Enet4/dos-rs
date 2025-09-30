//! conio.h

use core::ffi::{c_char, c_int, c_uchar, c_void};

pub const _NOCURSOR: c_int = 0;
pub const _SOLIDCURSOR: c_int = 1;
pub const _NORMALCURSOR: c_int = 2;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct text_info {
    pub winleft: c_uchar,
    pub wintop: c_uchar,
    pub winright: c_uchar,
    pub winbottom: c_uchar,
    pub attribute: c_uchar,
    pub normattr: c_uchar,
    pub currmode: c_uchar,
    pub screenheight: c_uchar,
    pub screenwidth: c_uchar,
    pub curx: c_uchar,
    pub cury: c_uchar,
}

pub mod text_modes {
    use core::ffi::c_int;

    pub const LASTMODE: c_int = -1;
    pub const BW40: c_int = 0;
    pub const C40: c_int = 1;
    pub const BW80: c_int = 2;
    pub const C80: c_int = 3;
    pub const MONO: c_int = 7;
    pub const C4350: c_int = 64;
}

#[allow(non_snake_case)]
pub mod COLORS {
    use core::ffi::c_int;

    /*  dark colors     */
    pub const BLACK: c_int = 0;
    pub const BLUE: c_int = 1;
    pub const GREEN: c_int = 2;
    pub const CYAN: c_int = 3;
    pub const RED: c_int = 4;
    pub const MAGENTA: c_int = 5;
    pub const BROWN: c_int = 6;
    pub const LIGHTGRAY: c_int = 7;
    /*  light colors    */
    pub const DARKGRAY: c_int = 8; /* "light black" */
    pub const LIGHTBLUE: c_int = 9;
    pub const LIGHTGREEN: c_int = 10;
    pub const LIGHTCYAN: c_int = 11;
    pub const LIGHTRED: c_int = 12;
    pub const LIGHTMAGENTA: c_int = 13;
    pub const YELLOW: c_int = 14;
    pub const WHITE: c_int = 15;
}

pub const BLINK: c_int = 0x80;

unsafe extern "C" {
    pub static mut directvideo: c_int;
    pub static mut _wscroll: c_int;

    pub fn blinkvideo();
    pub fn cgets(string: *mut c_char) -> *mut c_char;
    pub fn clreol();
    pub fn clrscr();
    pub fn _conio_kbhit() -> c_int; /* checks for ungetch char */
    pub fn cprintf(format: *const c_char, ...) -> c_int;
    pub fn cputs(string: *const c_char) -> c_int;
    pub fn cscanf(format: *const c_char, ...) -> c_int;
    pub fn delline();
    pub fn getch() -> c_int;
    pub fn getche() -> c_int;
    pub fn _conio_gettext(
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        destin: *mut c_void,
    ) -> c_int;
    pub fn gettextinfo(r: *mut text_info);
    pub fn gotoxy(x: c_int, y: c_int);
    pub fn gppconio_init();
    pub fn highvideo();
    pub fn insline();
    pub fn intensevideo();
    pub fn lowvideo();
    pub fn movetext(
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        destleft: c_int,
        desttop: c_int,
    ) -> c_int;
    pub fn normvideo();
    pub fn putch(c: c_int) -> c_int;
    pub fn puttext(
        left: c_int,
        top: c_int,
        right: c_int,
        bottom: c_int,
        source: *mut c_void,
    ) -> c_int;
    pub fn _setcursortype(cursor_type: c_int);
    pub fn _set_screen_lines(nlines: c_int);
    pub fn textattr(attr: c_int);
    pub fn textbackground(color: c_int);
    pub fn textcolor(color: c_int);
    pub fn textmode(mode: c_int);
    pub fn ungetch(ch: c_int) -> c_int;
    pub fn wherex() -> c_int;
    pub fn wherey() -> c_int;
    pub fn window(left: c_int, top: c_int, right: c_int, bottom: c_int);
}

pub use _conio_gettext as gettext;
pub use _conio_kbhit as kbhit;
