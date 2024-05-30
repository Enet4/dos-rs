//! Port control functions (as declared in `pc.h`).

extern "C" {
    pub fn inportb(_port: u16) -> u8;
    pub fn inportw(_port: u16) -> u16;
    pub fn inportl(_port: u16) -> u32;
    pub fn inportsb(_port: u16, _buf: *mut u8, _len: u32);
    pub fn inportsw(_port: u16, _buf: *mut u16, _len: u32);
    pub fn inportsl(_port: u16, _buf: *mut u32, _len: u32);
    pub fn outportb(_port: u16, _data: u8);
    pub fn outportw(_port: u16, _data: u16);
    pub fn outportl(_port: u16, _data: u32);
    pub fn outportsb(_port: u16, _buf: *const u8, _len: u32);
    pub fn outportsw(_port: u16, _buf: *const u16, _len: u32);
    pub fn outportsl(_port: u16, _buf: *const u32, _len: u32);
}
