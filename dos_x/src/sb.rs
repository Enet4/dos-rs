use djgpp::{
    dos::delay,
    pc::{inportb, outportb},
};

const SB_RESET: u16 = 0x6;
const SB_READ_DATA: u16 = 0xA;
const SB_READ_DATA_STATUS: u16 = 0xE;

unsafe fn reset_dsp(port: u16) -> u8 {
    outportb(port + SB_RESET, 1);
    delay(1);
    outportb(port + SB_RESET, 0);
    delay(1);

    let status = inportb(port + SB_READ_DATA_STATUS);
    if (status & 0x80) == 0x80 && inportb(port + SB_READ_DATA) == 0xAA {
        return 1;
    }

    0
}

pub fn detect_sb() -> Option<(u16, u8, u8)> {
    let mut addr = 0;
    for temp in [0x220, 0x230, 0x240, 0x250, 0x260, 0x270] {
        unsafe {
            let x = reset_dsp(temp);
            if x != 0 {
                addr = temp;
                break;
            }
        }
    }

    if addr == 0 {
        return None;
    }

    let env_blaster = unsafe { djgpp::stdlib::getenv(c"BLASTER".as_ptr()) };
    if env_blaster.is_null() {
        return None;
    }

    let env_blaster = unsafe { core::ffi::CStr::from_ptr(env_blaster) }.to_bytes();
    let mut dma = 0;
    let mut irq = 0;

    for i in 0..env_blaster.len() {
        if env_blaster[i] | 32 == b'd' {
            dma = env_blaster[i + 1] - b'0';
        } else if env_blaster[i] | 32 == b'i' {
            irq = env_blaster[i + 1] - b'0';
        }
    }

    if dma == 0 || irq == 0 {
        return None;
    }

    Some((addr, dma, irq))
}
