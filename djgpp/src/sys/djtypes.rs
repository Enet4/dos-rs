#![allow(non_camel_case_types)]
use core::ffi::{c_int, c_long, c_longlong, c_uint, c_ulong};

pub type clock_t = c_int;
pub type gid_t = c_int;
pub type off_t = c_int;
pub type off64_t = c_longlong;
pub type offset_t = c_longlong;
pub type pid_t = c_int;
pub type size_t = c_ulong;
pub type ssize_t = c_long;
pub type time_t = c_uint;
pub type uid_t = c_int;
