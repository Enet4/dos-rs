// nearptr.h

/* Functions to enable "near" pointer access to DOS memory under DPMI
   CW Sandmann 7-95  NO WARRANTY: WARNING, since these functions disable
   memory protection, they MAY DESTROY EVERYTHING ON YOUR COMPUTER!
*/

use core::ffi::c_int;

extern "C" {
    /** Returns 0 if feature not avail */
    pub fn __djgpp_nearptr_enable() -> c_int;
    /** Enables protection */
    pub fn __djgpp_nearptr_disable();

    /* Limit on CS and on DS if prot */
    pub static __djgpp_selector_limit: c_int;
    /* Used in calculation below */
    pub static __djgpp_base_address: c_int;
}

#[macro_export]
macro_rules! djgpp_conventional_base {
    () => {
        (-__djgpp_base_address) as *const u8 as *const _
    };
    ($addr: expr) => {
        $addr + djgpp_conventional_base!()
    };
}
