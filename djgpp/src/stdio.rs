//! stdio.h
//!

#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_char = i8;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct FILE {
    _opaque: [u8; 0],
}

extern "C" {
    pub fn clearerr(stream: *mut FILE);
    pub fn fclose(stream: *mut FILE) -> c_int;
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn fflush(stream: *mut FILE) -> c_int;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn fgetpos(stream: *mut FILE, pos: *mut c_int) -> c_int;
    pub fn fgets(s: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn fread(ptr: *mut c_char, size: c_int, nelem: c_int, stream: *mut FILE) -> c_int;
    pub fn freopen(filename: *const c_char, mode: *const c_char, stream: *mut FILE) -> *mut FILE;
    pub fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn fseek(stream: *mut FILE, offset: c_int, mode: c_int) -> c_int;
    pub fn fsetpos(stream: *mut FILE, pos: *mut c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_int;
    pub fn fwrite(ptr: *const c_char, size: c_int, nelem: c_int, stream: *mut FILE) -> c_int;
    pub fn getc(stream: *mut FILE) -> c_int;
    pub fn getchar() -> c_int;
    pub fn gets(s: *mut c_char) -> *mut c_char;
    pub fn perror(s: *const c_char);
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn putc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn remove(filename: *const c_char) -> c_int;
    pub fn rename(old: *const c_char, new: *const c_char) -> c_int;
    pub fn rewind(stream: *mut FILE);
    pub fn scanf(format: *const c_char, ...) -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn setvbuf(stream: *mut FILE, buf: *mut c_char, mode: c_int, size: usize) -> c_int;
    pub fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn tmpnam(s: *mut c_char) -> *mut c_char;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn vfprintf(stream: *mut FILE, format: *const c_char, ap: *mut c_int) -> c_int;
    pub fn vprintf(format: *const c_char, ap: *mut c_int) -> c_int;
    pub fn vsprintf(s: *mut c_char, format: *const c_char, ap: *mut c_int) -> c_int;

    pub fn snprintf(str: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    pub fn vfscanf(stream: *mut FILE, format: *const c_char, ap: *mut c_int) -> c_int;
    pub fn vscanf(format: *const c_char, ap: *mut c_int) -> c_int;
    pub fn vsnprintf(str: *mut c_char, n: usize, fmt: *const c_char, ap: *mut c_int) -> c_int;
    pub fn vsscanf(s: *const c_char, format: *const c_char, ap: *mut c_int) -> c_int;

    pub fn dprintf(_fd: c_int, _format: *const c_char) -> c_int;
    pub fn fileno(_stream: *const FILE) -> c_int;
    pub fn fdopen(_fildes: c_int, _type: *const c_char) -> *mut FILE;
    pub fn mkstemp(_template: *mut c_char) -> c_int;
    pub fn pclose(_pf: *mut FILE) -> c_int;
    pub fn popen(_command: *const c_char, _mode: *const c_char) -> *mut FILE;
    pub fn tempnam(_dir: *const c_char, _prefix: *const c_char) -> *mut c_char;
}
