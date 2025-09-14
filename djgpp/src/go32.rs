//! GO32 functions (as declared in `go32.h`).

/* These lengths are in bytes, optimized for speed */

unsafe extern "C" {
    pub fn dosmemget(_offset: u32, _length: usize, _buffer: *mut u8);
    pub fn dosmemput(_buffer: *const u8, _length: usize, _offset: u32);
}

/* The lengths here are in TRANSFERS, not bytes! */

unsafe extern "C" {
    pub fn _dosmemgetb(_offset: u32, _xfers: usize, _buffer: *mut u8);
    pub fn _dosmemgetw(_offset: u32, _xfers: usize, _buffer: *mut u8);
    pub fn _dosmemgetl(_offset: u32, _xfers: usize, _buffer: *mut u8);
    pub fn _dosmemputb(_buffer: *const u8, _xfers: usize, _offset: u32);
    pub fn _dosmemputw(_buffer: *const u8, _xfers: usize, _offset: u32);
    pub fn _dosmemputl(_buffer: *const u8, _xfers: usize, _offset: u32);
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct __Go32_Info_Block {
    pub size_of_this_structure_in_bytes: u32,
    pub linear_address_of_primary_screen: u32,
    pub linear_address_of_secondary_screen: u32,
    pub linear_address_of_transfer_buffer: u32,
    pub size_of_transfer_buffer: u32, /* >= 4k */
    pub pid: u32,
    pub master_interrupt_controller_base: u8,
    pub slave_interrupt_controller_base: u8,
    pub selector_for_linear_memory: u16,
    pub linear_address_of_stub_info_structure: u32,
    pub linear_address_of_original_psp: u32,
    pub run_mode: u16,
    pub run_mode_info: u16,
}

unsafe extern "C" {
    pub static _go32_info_block: __Go32_Info_Block;
}

#[macro_export]
macro_rules! _dos_ds {
    () => {
        djgpp::go32::_go32_info_block.selector_for_linear_memory
    };
}
