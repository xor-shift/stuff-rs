use std::default::Default;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Display for Color {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
}

impl Default for Color {
    fn default() -> Self { Self { r: 0, g: 0, b: 0, a: 255 } }
}

impl Color {
    pub const fn from_rgba_bytes(channels: [u8; 4]) -> Self {
        Self {
            r: channels[0],
            g: channels[1],
            b: channels[2],
            a: channels[3],
        }
    }

    pub const fn from_rgb_bytes(channels: [u8; 3]) -> Self {
        Self {
            r: channels[0],
            g: channels[1],
            b: channels[2],
            a: 255,
        }
    }

    pub const fn from_u32(color_code: u32) -> Self {
        let bytes = color_code.to_be_bytes();
        Self::from_rgba_bytes(bytes)
    }

    pub const fn hash(self) -> u8 {
        let ret = 0u8 //
            .wrapping_add(self.r.wrapping_mul(3))
            .wrapping_add(self.g.wrapping_mul(5))
            .wrapping_add(self.b.wrapping_mul(7))
            .wrapping_add(self.a.wrapping_mul(11));

        return ret % 64;
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

#[derive(Clone)]
pub struct ColorHashTable {
    data: [Color; 64],
}

impl Default for ColorHashTable {
    fn default() -> Self { Self::new() }
}

impl ColorHashTable {
    pub const fn new() -> Self { Self { data: [Color::from_u32(0); 64] } }

    pub fn remember(&mut self, color: Color) {
        self.data[color.hash() as usize] = color;
    }

    pub fn recall(&self, idx: u8) -> Color {
        assert!(idx < 64, "index too large");

        self.data[idx as usize]
    }
}
