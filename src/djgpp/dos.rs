//! dos.h

extern "C" {
    pub fn _get_dos_version(x: int) -> u16;
    pub fn _get_fat_size(_drive: i32) -> i32;
    pub fn _get_fs_type(_drive: i32, _result_str: *const u8) -> i32;
    pub fn _is_cdrom_drive(_drive: i32) -> i32;
    pub fn _is_fat32(_drive: i32) -> i32;
    pub fn _is_ram_drive(_drive: i32) -> i32;
    pub fn _media_type(_drive: i32) -> i32;

    pub fn delay(_msec: u32);
}
