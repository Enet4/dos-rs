//! DPMI functions (as declared in `dpmi.h`).
#![allow(non_camel_case_types)]

pub type __dpmi_error = u16;

// Rust:

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dpmi_regs_d {
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub res: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dpmi_regs_x {
    pub di: u16,
    pub di_hi: u16,
    pub si: u16,
    pub si_hi: u16,
    pub bp: u16,
    pub bp_hi: u16,
    pub res: u16,
    pub res_hi: u16,
    pub bx: u16,
    pub bx_hi: u16,
    pub dx: u16,
    pub dx_hi: u16,
    pub cx: u16,
    pub cx_hi: u16,
    pub ax: u16,
    pub ax_hi: u16,
    pub flags: u16,
    pub es: u16,
    pub ds: u16,
    pub fs: u16,
    pub gs: u16,
    pub ip: u16,
    pub cs: u16,
    pub sp: u16,
    pub ss: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dpmi_regs_h {
    pub edi: [u8; 4],
    pub esi: [u8; 4],
    pub ebp: [u8; 4],
    pub res: [u8; 4],
    pub bl: u8,
    pub bh: u8,
    pub ebx_b2: u8,
    pub ebx_b3: u8,
    pub dl: u8,
    pub dh: u8,
    pub edx_b2: u8,
    pub edx_b3: u8,
    pub cl: u8,
    pub ch: u8,
    pub ecx_b2: u8,
    pub ecx_b3: u8,
    pub al: u8,
    pub ah: u8,
    pub eax_b2: u8,
    pub eax_b3: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union __dpmi_regs {
    pub d: __dpmi_regs_d,
    pub x: __dpmi_regs_x,
    pub h: __dpmi_regs_h,
}

extern "C" {
    pub fn __dpmi_allocate_dos_memory(size: u32, segment: *mut u16) -> __dpmi_error;

    pub fn __dpmi_free_dos_memory(segment: u16) -> __dpmi_error;

    pub fn __dpmi_simulate_real_mode_procedure(
        segment: u16,
        offset: u16,
        registers: *mut __dpmi_regs,
    ) -> __dpmi_error;

    pub fn __dpmi_int(int: u8, registers: *mut __dpmi_regs) -> __dpmi_error;

    pub fn __dpmi_yield() -> __dpmi_error;
}
