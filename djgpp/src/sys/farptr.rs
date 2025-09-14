// farptr.h

unsafe extern "C" {
    pub fn _farpokeb(selector: u16, offset: u32, value: u8);
    pub fn _farpokew(selector: u16, offset: u32, value: u16);
    pub fn _farpokel(selector: u16, offset: u32, value: u32);
    pub fn _farpeekb(selector: u16, offset: u32) -> u8;
    pub fn _farpeekw(selector: u16, offset: u32) -> u16;
    pub fn _farpeekl(selector: u16, offset: u32) -> u32;
    pub fn _farsetsel(selector: u16);
    pub fn _fargetsel() -> u16;
    pub fn _farnspokeb(addr: u32, value: u8);
    pub fn _farnspokew(addr: u32, value: u16);
    pub fn _farnspokel(addr: u32, value: u32);
    pub fn _farnspeekb(addr: u32) -> u8;
    pub fn _farnspeekw(addr: u32) -> u16;
    pub fn _farnspeekl(addr: u32) -> u32;
}
