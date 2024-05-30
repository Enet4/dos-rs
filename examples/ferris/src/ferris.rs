use dos_x::vga::Palette;

static FERRIS_TIF_DATA: &[u8] = include_bytes!("../assets/ferris.tif");

pub fn ferris_pixel_data() -> &'static [u8] {
    &FERRIS_TIF_DATA[0x08..0x0_fa08]
}

pub fn ferris_color_palette() -> Palette {
    // ColorMap tag begins @ 0x0_faca
    // ID: 320 (0x0140)
    // Type: SHORT (3)
    // Count: 768 (0x0300)
    // Tag data address: 0x0_fb0a
    let palette_offset = 0x0_fb0a;

    let palette_size = 256 * 3 * 2;
    let g_offset = 256 * 2;
    let b_offset = 256 * 4;

    // color table in tif is channel-contiguous
    let palette_r = &FERRIS_TIF_DATA[palette_offset..palette_offset + g_offset];
    let palette_g = &FERRIS_TIF_DATA[palette_offset + g_offset..palette_offset + b_offset];
    let palette_b = &FERRIS_TIF_DATA[palette_offset + b_offset..palette_offset + palette_size];

    let mut palette = [0xffu8; 768];

    // convert to standard layout
    // and narrow it down to 6 bits per channel
    for i in 0..256 {
        // R channel
        palette[i * 3] = palette_r[i * 2] >> 2;
        // G channel
        palette[i * 3 + 1] = palette_g[i * 2] >> 2;
        // B channel
        palette[i * 3 + 2] = palette_b[i * 2] >> 2;
    }

    Palette::new(palette)
}
