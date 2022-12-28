#[derive(Clone, Copy, Debug)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    White,
    Black,
    RGB(u8, u8, u8),
}

impl Color {
    pub fn get_rgb(&self) -> [u8; 3] {
        match self {
            Color::Red => [0xff, 0x00, 0x00],
            Color::Orange => [0xff, 0xaa, 0x00],
            Color::Yellow => [0xff, 0xff, 0x00],
            Color::Green => [0x00, 0xff, 0x00],
            Color::Blue => [0x00, 0x00, 0xff],
            Color::Purple => [0xaa, 0x00, 0xaa],
            Color::White => [0xff, 0xff, 0xff],
            Color::Black => [0x00, 0x00, 0x00],
            Color::RGB(r, g, b) => [*r, *g, *b],
        }
    }
}