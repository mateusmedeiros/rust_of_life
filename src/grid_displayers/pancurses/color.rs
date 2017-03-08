pub enum Color {
    Hex(u32),
    RGB(u8, u8, u8)
}

#[inline]
fn color_to_rgb_tuple(color: &Color) -> (i16, i16, i16) {
    match *color {
        Color::Hex(hex_color) => (
            ((hex_color >> 16) & 0xFF) as i16,
            ((hex_color >> 8) & 0xFF) as i16,
            ((hex_color) & 0xFF) as i16
        ),
        Color::RGB(r, g, b) => (r as i16, g as i16, b as i16)
    }
}

impl<'a> From<&'a Color> for (i16, i16, i16) {
    fn from(color: &'a Color) -> (i16, i16, i16) {
        color_to_rgb_tuple(color)
    }
}

impl From<Color> for (i16, i16, i16) {
    fn from(color: Color) -> (i16, i16, i16) {
        color_to_rgb_tuple(&color)
    }
}

impl From<(i16, i16, i16)> for Color {
    fn from((r, g, b): (i16, i16, i16)) -> Color {
        Color::RGB(
            if (r < 0) { 0 } else { r as u8 },
            if (g < 0) { 0 } else { g as u8 },
            if (b < 0) { 0 } else { b as u8 }
        )
    }
}
