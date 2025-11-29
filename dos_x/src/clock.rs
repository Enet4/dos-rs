//! Module for retrieving the system clock time in DOS

/// system clock memory location
const SYSTEM_CLOCK_ADDR: u32 = 0x046C;

/// Obtain the clock tick counter from the system clock address.
pub fn get_system_clock_ticks() -> u32 {
    unsafe {
        let mut buffer = [0; 4];
        djgpp::go32::_dosmemgetl(SYSTEM_CLOCK_ADDR, 1, buffer.as_mut_ptr());
        u32::from_ne_bytes(buffer)
    }
}
