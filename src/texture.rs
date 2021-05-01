pub struct Font {
    pub charset_length: usize,
    pub glyph_size: usize,
    pub glyphs: Vec<bool>,
}

impl Font {
    pub fn load_from_bmp(bmp_data: &Vec<u8>, glyph_size: usize) -> Font {
        let data_position = u32::from_le_bytes([
            bmp_data[0x0A],
            bmp_data[0x0B],
            bmp_data[0x0C],
            bmp_data[0x0D],
        ]);
        let mut glyphs = Vec::new();
        // step_by 3 assuming 24-bit depth
        for byte_index in ((data_position as usize)..bmp_data.len()).step_by(3) {
            glyphs.push(bmp_data[byte_index] == 0xFF);
        }
        Font {
            charset_length: 256,
            glyph_size,
            glyphs,
        }
    }
}

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub has_transparency: bool,
    pub data: Vec<u32>,
}

impl Texture {
    pub fn load_from_bmp(bmp_data: &Vec<u8>) -> Texture {
        let data_position = u32::from_le_bytes([
            bmp_data[0x0A],
            bmp_data[0x0B],
            bmp_data[0x0C],
            bmp_data[0x0D],
        ]);
        // assuming windows BITMAPINFOHEADER, these are i32
        let width = i32::from_le_bytes([
            bmp_data[0x12],
            bmp_data[0x13],
            bmp_data[0x14],
            bmp_data[0x15],
        ]) as usize;
        let height = i32::from_le_bytes([
            bmp_data[0x16],
            bmp_data[0x17],
            bmp_data[0x18],
            bmp_data[0x19],
        ]) as usize;
        let mut has_transparency = false;
        let mut data = Vec::with_capacity(width * height);
        // step_by 3 assuming 24-bit depth
        for byte_index in ((data_position as usize)..bmp_data.len()).step_by(3) {
            if bmp_data[byte_index] == 0x00
                && bmp_data[byte_index + 1] == 0x00
                && bmp_data[byte_index + 2] == 0x00
            {
                has_transparency = true;
            }
            data.push(u32::from_le_bytes([
                bmp_data[byte_index],
                bmp_data[byte_index + 1],
                bmp_data[byte_index + 2],
                0x00,
            ]));
        }
        data.reverse();
        Texture {
            width,
            height,
            has_transparency,
            data,
        }
    }
}
