//! DPMI functions (as declared in `dpmi.h`).
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    pub static mut __dpmi_error: u16;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dpmi_raddr {
    offset16: u16,
    segment: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dpmi_paddr {
    offset32: u32,
    selector: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dpmi_meminfo {
    /* 0, 2 */
    handle: u32,
    /* or count */	/* 4, 6 */
    size: u32,
    /* 8, 10 */
    address: u32,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dpmi_version_ret {
    pub major: u8,
    pub minor: u8,
    pub flags: u16,
    pub cpu: u8,
    pub master_pic: u8,
    pub slave_pic: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dpmi_free_mem_info {
    pub largest_available_free_block_in_bytes: u32,
    pub maximum_unlocked_page_allocation_in_pages: u32,
    pub maximum_locked_page_allocation_in_pages: u32,
    pub linear_address_space_size_in_pages: u32,
    pub total_number_of_unlocked_pages: u32,
    pub total_number_of_free_pages: u32,
    pub total_number_of_physical_pages: u32,
    pub free_linear_address_space_in_pages: u32,
    pub size_of_paging_file_partition_in_pages: u32,
    pub reserved: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __dpmi_memory_info {
    pub total_allocated_bytes_of_physical_memory_host: u32,
    pub total_allocated_bytes_of_virtual_memory_host: u32,
    pub total_available_bytes_of_virtual_memory_host: u32,
    pub total_allocated_bytes_of_virtual_memory_vcpu: u32,
    pub total_available_bytes_of_virtual_memory_vcpu: u32,
    pub total_allocated_bytes_of_virtual_memory_client: u32,
    pub total_available_bytes_of_virtual_memory_client: u32,
    pub total_locked_bytes_of_memory_client: u32,
    pub max_locked_bytes_of_memory_client: u32,
    pub highest_linear_address_available_to_client: u32,
    pub size_in_bytes_of_largest_free_memory_block: u32,
    pub size_of_minimum_allocation_unit_in_bytes: u32,
    pub size_of_allocation_alignment_unit_in_bytes: u32,
    pub reserved: [u32; 19],
}

unsafe extern "C" {
    pub fn __dpmi_yield();

    pub fn __dpmi_allocate_ldt_descriptors(count: c_int) -> c_int; /* DPMI 0.9 AX=0000 */
    pub fn __dpmi_free_ldt_descriptor(_descriptor: c_int) -> c_int;	/* DPMI 0.9 AX=0001 */
    pub fn __dpmi_segment_to_descriptor(_segment: c_int) -> c_int;	/* DPMI 0.9 AX=0002 */
    pub fn __dpmi_get_selector_increment_value() -> c_int;	/* DPMI 0.9 AX=0003 */
    pub fn __dpmi_get_segment_base_address(_selector: c_int, _addr: *mut u32) -> c_int;	/* DPMI 0.9 AX=0006 */
    pub fn __dpmi_set_segment_base_address(_selector: c_int, _address: u32) -> c_int;	/* DPMI 0.9 AX=0007 */
    pub fn __dpmi_get_segment_limit(_selector: c_int) -> u32;    /* LSL instruction  */
    pub fn __dpmi_set_segment_limit(_selector: c_int, _limit: u32) -> c_int;	/* DPMI 0.9 AX=0008 */

    pub fn __dpmi_allocate_dos_memory(size: u32, segment: *mut u16) -> c_int;   /* DPMI 0.9 AX=0100 */
    pub fn __dpmi_free_dos_memory(segment: u16) -> c_int;   /* DPMI 0.9 AX=0101 */
    pub fn __dpmi_resize_dos_memory(_selector: c_int, _newpara: c_int, _ret_max: *mut c_int) -> c_int;	/* DPMI 0.9 AX=0102 */

    pub fn __dpmi_get_real_mode_interrupt_vector(
        _vector: c_int,
        _address: *mut __dpmi_raddr,
    ) -> c_int; /* DPMI 0.9 AX=0200 */
    pub fn __dpmi_set_real_mode_interrupt_vector(
        _vector: c_int,
        _address: *mut __dpmi_raddr,
    ) -> c_int; /* DPMI 0.9 AX=0201 */
    pub fn __dpmi_get_processor_exception_handler_vector(
        _vector: c_int,
        _address: *mut __dpmi_paddr,
    ) -> c_int; /* DPMI 0.9 AX=0202 */
    pub fn __dpmi_set_processor_exception_handler_vector(
        _vector: c_int,
        _address: *mut __dpmi_paddr,
    ) -> c_int; /* DPMI 0.9 AX=0203 */
    pub fn __dpmi_get_protected_mode_interrupt_vector(
        _vector: c_int,
        _address: *mut __dpmi_paddr,
    ) -> c_int; /* DPMI 0.9 AX=0204 */
    pub fn __dpmi_set_protected_mode_interrupt_vector(
        _vector: c_int,
        _address: *mut __dpmi_paddr,
    ) -> c_int; /* DPMI 0.9 AX=0205 */

    pub fn __dpmi_simulate_real_mode_interrupt(_vector: c_int, _regs: *mut __dpmi_regs) -> c_int;	/* DPMI 0.9 AX=0300 */
    pub fn __dpmi_int(int: u8, registers: *mut __dpmi_regs) -> c_int;   /* like above, but sets ss sp fl */	/* DPMI 0.9 AX=0300 */

    pub fn __dpmi_get_version(_ret: *mut __dpmi_version_ret) -> c_int;   /* DPMI 0.9 AX=0400 */

    pub fn __dpmi_get_capabilities(_flags: *mut c_int, vendor_info: *mut c_char);	/* DPMI 1.0 AX=0401 */

    pub fn __dpmi_get_free_memory_information(_info: *mut __dpmi_free_mem_info) -> c_int;	/* DPMI 0.9 AX=0500 */
    pub fn __dpmi_allocate_memory(_info: *mut __dpmi_meminfo) -> c_int;	/* DPMI 0.9 AX=0501 */
    pub fn __dpmi_free_memory(_handle: u32) -> c_int;	/* DPMI 0.9 AX=0502 */
    pub fn __dpmi_resize_memory(_info: *mut __dpmi_meminfo) -> c_int;	/* DPMI 0.9 AX=0503 */

    pub fn __dpmi_allocate_linear_memory(_info: *mut __dpmi_meminfo, _commit: c_int) -> c_int;	/* DPMI 1.0 AX=0504 */
    pub fn __dpmi_resize_linear_memory(_info: *mut __dpmi_meminfo, _commit: c_int) -> c_int;	/* DPMI 1.0 AX=0505 */

    /* These next four functions return the old state */
    pub fn __dpmi_get_and_disable_virtual_interrupt_state() -> c_int;	/* DPMI 0.9 AX=0900 */
    pub fn __dpmi_get_and_enable_virtual_interrupt_state() -> c_int;	/* DPMI 0.9 AX=0901 */
    pub fn __dpmi_get_and_set_virtual_interrupt_state(_old_state: c_int) -> c_int;	/* DPMI 0.9 AH=09   */
    pub fn __dpmi_get_virtual_interrupt_state() -> c_int;   /* DPMI 0.9 AX=0902 */

    pub fn __dpmi_get_coprocessor_status() -> c_int;	/* DPMI 1.0 AX=0e00 */
    pub fn __dpmi_set_coprocessor_emulation(_flags: c_int) -> c_int;	/* DPMI 1.0 AX=0e01 */

    pub fn __dpmi_simulate_real_mode_procedure(
        segment: u16,
        offset: u16,
        registers: *mut __dpmi_regs,
    ) -> c_int;


}

/* Backwards compatibility stuff  */

pub type _go32_dpmi_registers = __dpmi_regs;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _go32_dpmi_meminfo {
    pub available_memory: u32,
    pub available_pages: u32,
    pub available_lockable_pages: u32,
    pub linear_space: u32,
    pub unlocked_pages: u32,
    pub available_physical_pages: u32,
    pub total_physical_pages: u32,
    pub free_linear_space: u32,
    pub max_pages_in_paging_file: u32,
    pub reserved: [u32; 3],
}

pub unsafe fn _go32_dpmi_get_free_memory_information(info: *mut _go32_dpmi_meminfo) -> c_int {
    unsafe {
        __dpmi_get_free_memory_information(info as *mut __dpmi_free_mem_info)
    }
}

pub unsafe fn _go32_dpmi_simulate_int(vector: c_int, regs: *mut _go32_dpmi_registers) -> c_int {
    unsafe {
        __dpmi_simulate_real_mode_interrupt(vector, regs as *mut __dpmi_regs)
    }
}

pub unsafe fn _go32_dpmi_simulate_fcall(
    segment: u16,
    offset: u16,
    registers: *mut _go32_dpmi_registers,
) -> c_int {
    unsafe {
        __dpmi_simulate_real_mode_procedure(segment, offset, registers as *mut __dpmi_regs)
    }
}
pub unsafe fn _go32_dpmi_simulate_fcall_iret(
    segment: u16,
    offset: u16,
    registers: *mut _go32_dpmi_registers,
) -> c_int {
    unsafe {
        __dpmi_simulate_real_mode_procedure(segment, offset, registers as *mut __dpmi_regs)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _go32_dpmi_seginfo {
    pub size: u32,
    pub pm_offset: u32,
    pub pm_selector: u16,
    pub rm_offset: u16,
    pub rm_segment: u16,
}

unsafe extern "C" {
    /* returns zero if success, else dpmi error and info->size is max size */
    pub fn _go32_dpmi_allocate_dos_memory(info: *mut _go32_dpmi_seginfo) -> c_int;
    /* set size to bytes/16, call, use rm_segment.  Do not
    change anthing but size until the memory is freed.
    If error, max size is returned in size as bytes/16. */
    pub fn _go32_dpmi_free_dos_memory(info: *mut _go32_dpmi_seginfo) -> c_int;
    /* set new size to bytes/16, call.  If error, max size
    is returned in size as bytes/16 */
    pub fn _go32_dpmi_resize_dos_memory(info: *mut _go32_dpmi_seginfo) -> c_int;
        /* uses pm_selector to free memory */

    /* These both use the rm_segment:rm_offset fields only */
    pub fn _go32_dpmi_get_real_mode_interrupt_vector(vector: c_int, info: *mut _go32_dpmi_seginfo) -> c_int;
    pub fn _go32_dpmi_set_real_mode_interrupt_vector(vector: c_int, info: *mut _go32_dpmi_seginfo) -> c_int;

    /* These do NOT wrap the function in pm_offset in an iret handler.
    You must provide an assembler interface yourself, or alloc one below.
    You may NOT longjmp out of an interrupt handler. */
    pub fn _go32_dpmi_get_protected_mode_interrupt_vector(vector: c_int, info: *mut _go32_dpmi_seginfo) -> c_int;
        /* puts vector in pm_selector:pm_offset. */
    pub fn _go32_dpmi_set_protected_mode_interrupt_vector(vector: c_int, info: *mut _go32_dpmi_seginfo) -> c_int;
        /* sets vector from pm_offset and pm_selector */

    /********** HELPER FUNCTIONS **********/

    pub fn _go32_dpmi_chain_protected_mode_interrupt_vector(vector: c_int, info: *mut _go32_dpmi_seginfo) -> c_int;

    /* locks memory from a specified offset within the code/data selector */
    pub fn _go32_dpmi_lock_code(_lockaddr: *mut c_void, _locksize: u32) -> c_int;
    pub fn _go32_dpmi_lock_data(_lockaddr: *mut c_void, _locksize: u32) -> c_int;
}


