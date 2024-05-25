
use djgpp::pc::{inportb, outportb};

/// The default Adlib address port
const ADLIB_DEFAULT_ADDR: u16 = 0x0388;
/// The default Adlib data port
const ADLIB_DEFAULT_DATA: u16 = 0x0389;

/// The default Adlib address port for the left speaker
const ADLIB_DEFAULT_L_ADDR: u16 = 0x0220;
/// The default Adlib data port for the left speaker
const ADLIB_DEFAULT_L_DATA: u16 = 0x0221;

/// The default Adlib address port for the right speaker
const ADLIB_DEFAULT_R_ADDR: u16 = 0x0222;
/// The default Adlib data port for the right speaker
const ADLIB_DEFAULT_R_DATA: u16 = 0x0223;

pub fn detect_adlib() -> u8 {

    unsafe {
        // 1) reset both timers
        write_command(0x04, 0x60);
        // 2) enable the interrupts
        write_command(0x04, 0x80);

        // 3) read status register
        let status = inportb(ADLIB_DEFAULT_ADDR);

        // 4) write FFh to register 2 (Timer 1)
        write_command(0x02, 0xff);

        // 5) start timer 1 by writing 21h to register 4
        write_command(0x04, 0x21);

        // 6) delay for at least 80 microseconds
        djgpp::dos::delay(1);

        // 7) read the status again
        let status2 = inportb(ADLIB_DEFAULT_ADDR);

        // 8) reset both timers and interrupts
        write_command(0x04, 0x60);

        // 9) check if est the stored results of steps 3 and 7 by ANDing them
        //    with E0h.  The result of step 3 should be 00h, and the 
        //    result of step 7 should be C0h.  If both are correct, an
        //    AdLib-compatible board is installed in the computer.
        if (status & 0xe0) == 0 && (status2 & 0xe0) == 0xc0 {
            return 1;
        }
    }

    0
}

/// Send an OPL command
pub unsafe fn write_command(register: u8, data: u8) {
    outportb(ADLIB_DEFAULT_ADDR, register);
    outportb(ADLIB_DEFAULT_DATA, data);
}

/// Send an OPL command to the left speaker
pub unsafe fn write_command_l(register: u8, data: u8) {
    outportb(ADLIB_DEFAULT_L_ADDR, register);
    outportb(ADLIB_DEFAULT_L_DATA, data);
}

/// Send an OPL command to the right speaker
pub unsafe fn write_command_r(register: u8, data: u8) {
    outportb(ADLIB_DEFAULT_R_ADDR, register);
    outportb(ADLIB_DEFAULT_R_DATA, data);
}

pub fn reset_adlib() {
    unsafe {
        write_command(0x01, 0x20);
        write_command(0xb0, 0x00);
    }
}
