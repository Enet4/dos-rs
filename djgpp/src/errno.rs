use core::ffi::{c_char, c_int};

extern "C" {
    pub static errno: c_int;
    pub static sys_errlist: *const *const c_char;
    pub static sys_nerr: c_int;
    pub(crate) static __sys_errlist: *const *const c_char;
    pub(crate) static __sys_nerr: c_int;
    pub(crate) static _doserrno: c_int;
}

pub const EDOM: i32 = 1;
pub const ERANGE: i32 = 2;

pub const E2BIG: i32 = 3;
pub const EACCES: i32 = 4;
pub const EAGAIN: i32 = 5;
pub const EBADF: i32 = 6;
pub const EBUSY: i32 = 7;
pub const ECHILD: i32 = 8;
pub const EDEADLK: i32 = 9;
pub const EEXIST: i32 = 10;
pub const EFAULT: i32 = 11;
pub const EFBIG: i32 = 12;
pub const EINTR: i32 = 13;
pub const EINVAL: i32 = 14;
pub const EIO: i32 = 15;
pub const EISDIR: i32 = 16;
pub const EMFILE: i32 = 17;
pub const EMLINK: i32 = 18;
pub const ENAMETOOLONG: i32 = 19;
pub const ENFILE: i32 = 20;
pub const ENODEV: i32 = 21;
pub const ENOENT: i32 = 22;
pub const ENOEXEC: i32 = 23;
pub const ENOLCK: i32 = 24;
pub const ENOMEM: i32 = 25;
pub const ENOSPC: i32 = 26;
pub const ENOSYS: i32 = 27;
pub const ENOTDIR: i32 = 28;
pub const ENOTEMPTY: i32 = 29;
pub const ENOTTY: i32 = 30;
pub const ENXIO: i32 = 31;
pub const EPERM: i32 = 32;
pub const EPIPE: i32 = 33;
pub const EROFS: i32 = 34;
pub const ESPIPE: i32 = 35;
pub const ESRCH: i32 = 36;
pub const EXDEV: i32 = 37;

pub const ENMFILE: i32 = 38;
pub const ELOOP: i32 = 39;
pub const EOVERFLOW: i32 = 40;
