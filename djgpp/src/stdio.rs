//! stdio.h
//! 

#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_char = i8;

extern "C" {
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
}
