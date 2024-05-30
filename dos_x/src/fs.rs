//! File system access under DOS
//!
//! This module is an abstraction similar to
//! the one found in the Rust standard library,
//! but designed for MS-DOS via DJGPP.

use core::ffi::{c_char, c_int, CStr};

use alloc::vec::Vec;
use djgpp::{
    stdio::{clearerr, feof, fflush, fileno, fopen, fread, FILE},
    sys::stat::{fstat, Stat},
};

use crate::djgpp_try;
use crate::io::Error;

#[derive(Debug)]
pub struct File {
    inner: *mut FILE,
}

pub type Result<T, E = crate::io::Error> = core::result::Result<T, E>;

impl File {
    /// Attempts to open a file in read-only mode.
    pub fn open(path: impl AsRef<CStr>) -> Result<Self> {
        Self::create_impl(path, b"rb\0".as_ptr() as *const _)
    }

    pub fn create(path: impl AsRef<CStr>) -> Result<Self> {
        Self::create_impl(path, b"wb\0".as_ptr() as *const _)
    }

    fn create_impl(path: impl AsRef<CStr>, mode: *const c_char) -> Result<Self> {
        let filename = path.as_ref();
        unsafe {
            let file = fopen(filename.as_ptr(), mode);
            if file.is_null() {
                return Err(Error::from_errno());
            }
            Ok(File { inner: file })
        }
    }

    /// Get the file descriptor
    fn fileno(&self) -> c_int {
        unsafe { fileno(self.inner) }
    }

    pub fn metadata(&self) -> Result<Metadata> {
        let mut stat = Stat::zeroed();
        unsafe {
            djgpp_try!(fstat(self.fileno(), &mut stat));
            Ok(Metadata { stat })
        }
    }

    /// Pull some bytes from this source into the specified buffer,
    /// returning how many bytes were read.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len() as c_int;
        unsafe {
            let ret = fread(buf.as_mut_ptr() as *mut _, 1, len, self.inner);
            if ret == len {
                return Ok(ret as usize);
            }
            // check if EOF
            let eof = feof(self.inner);
            let out = if eof != 0 {
                // ret contains the number of bytes read
                Ok(ret as usize)
            } else {
                // error
                Err(Error::from_errno())
            };
            clearerr(self.inner);
            out
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        unsafe {
            djgpp_try!(fflush(self.inner));
            Ok(())
        }
    }
}

pub fn read(path: impl AsRef<CStr>) -> Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let metadata = f.metadata()?;

    let file_size = metadata.file_size();
    let mut buf = alloc::vec![0u8; file_size];

    let mut rest = &mut buf[..];

    loop {
        let bytes_read = f.read(rest)?;
        if bytes_read == 0 {
            unsafe {
                let rest_len = rest.len();
                buf.set_len(buf.len() - rest_len);
                return Ok(buf);
            }
        }
        if bytes_read == rest.len() {
            return Ok(buf);
        }
        // update buffer rest and try again
        rest = &mut rest[bytes_read..];
    }
}

/// File metadata
#[derive(Debug)]
pub struct Metadata {
    stat: Stat,
}

impl Metadata {
    pub fn file_size(&self) -> usize {
        self.stat.st_size as usize
    }
}
