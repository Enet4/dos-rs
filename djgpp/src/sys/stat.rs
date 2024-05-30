use core::ffi::{c_char, c_int};

use super::{
    djtypes::{gid_t, off_t, time_t, uid_t},
    types::{blksize_t, dev_t, ino_t, mode_t, nlink_t},
};

#[repr(C)]
#[derive(Debug)]
pub struct Stat {
    pub st_atime: time_t,
    pub st_ctime: time_t,
    pub st_dev: dev_t,
    pub st_gid: gid_t,
    pub st_ino: ino_t,
    pub st_mode: mode_t,
    pub st_mtime: time_t,
    pub st_nlink: nlink_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_uid: uid_t,
    pub st_rdev: dev_t,
}

impl Stat {
    pub fn zeroed() -> Self {
        Stat {
            st_atime: 0,
            st_ctime: 0,
            st_dev: 0,
            st_gid: 0,
            st_ino: 0,
            st_mode: 0,
            st_mtime: 0,
            st_nlink: 0,
            st_size: 0,
            st_blksize: 0,
            st_uid: 0,
            st_rdev: 0,
        }
    }
}

extern "C" {
    pub fn chmod(path: *const c_char, _mode: mode_t) -> c_int;
    pub fn fchmod(_fildes: c_int, _mode: mode_t) -> c_int;
    pub fn fstat(_fildes: c_int, _buf: *mut Stat) -> c_int;
    pub fn mkdir(_path: *const c_char, _mode: mode_t) -> c_int;
    pub fn mkfifo(_path: *const c_char, _mode: mode_t) -> c_int;
    pub fn stat(_path: *const c_char, _buf: *mut Stat) -> c_int;
    pub fn umask(_cmask: mode_t) -> mode_t;
}
