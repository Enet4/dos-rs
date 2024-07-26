use core::ffi::{c_int, CStr};

use djgpp::errno::{sys_errlist, EIO};

#[derive(Copy, Clone, PartialEq)]
pub struct Error(c_int);

impl Error {
    pub const E_IO: Error = Error(EIO);
    pub const E_UNKNOWN: Error = Error(127);

    pub fn new(err: c_int) -> Option<Self> {
        if err != 0 {
            Some(Error(err))
        } else {
            None
        }
    }

    pub fn from_errno() -> Self {
        let err = unsafe { djgpp::errno::errno };
        Self::new(err).unwrap_or(Error::E_UNKNOWN)
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Error").field("code", &self.0).finish()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let code = self.0;
        if (0..=40).contains(&code) {
            // fetch message from errlist
            unsafe {
                let errmsg = *(sys_errlist.offset(code as isize));
                if !errmsg.is_null() {
                    let msg = CStr::from_ptr(errmsg).to_str().unwrap();
                    return write!(f, "{msg} (#{code})");
                }
            }
        }
        write!(f, "Unknown error (#{code})")
    }
}

#[macro_export(local_inner_macros)]
macro_rules! djgpp_try {
    ($e: expr) => {{
        let ret = $e;
        if ret != 0 {
            return core::result::Result::Err($crate::io::Error::from_errno());
        }
    }};
}

#[macro_export(local_inner_macros)]
macro_rules! println {
    ($template: literal) => {
        unsafe {
            let msg = core::concat!($template, "\n\0").as_ptr() as *const core::ffi::c_char;
            $crate::djgpp::stdio::printf(msg);
        }
    };
    ($template: literal, $($arg: expr),*) => {
        unsafe {
            let msg = alloc::format!(core::concat!($template, "\n\0"), $(&$arg),*);
            $crate::djgpp::stdio::printf(msg.as_ptr() as *const core::ffi::c_char);
        }
    };
}