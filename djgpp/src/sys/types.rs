#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_ulong};

pub type blkcnt_t = c_int;
pub type blksize_t = c_int;
pub type dev_t = c_int;
pub type fsblkcnt_t = c_ulong;
pub type fsfilcnt_t = c_ulong;
pub type ino_t = c_int;
pub type mode_t = c_int;
pub type nlink_t = c_int;
