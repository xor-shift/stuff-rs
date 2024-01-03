#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RawCommand {
    Index(u8),
    Diff([u8; 3]),
    Luma([u8; 3]),
    Run(u8),
    ColorRGB([u8; 3]),
    ColorRGBA([u8; 4]),
}

impl RawCommand {
    pub fn from_bytes(bytes: &[u8]) -> Option<(Self, &[u8])> {
        if let Some(&v) = bytes.first() {
            match v {
                0b0000_0000..=0b0011_1111 => Some((Self::Index(v), &bytes[1..])),
                0b0100_0000..=0b0111_1111 => Some((Self::Diff([(v & 0x30) >> 4, (v & 0x0C) >> 2, v & 0x3]), &bytes[1..])),
                0b1000_0000..=0b1011_1111 => Self::from_bytes_luma(bytes),
                0b1100_0000..=0b1111_1101 => Some((Self::Run(v & 0x3F), &bytes[1..])),
                0b1111_1110..=0b1111_1110 => Self::from_bytes_rgb(bytes),
                0b1111_1111..=0b1111_1111 => Self::from_bytes_rgba(bytes),
            }
        } else {
            None
        }
    }

    fn from_bytes_luma(bytes: &[u8]) -> Option<(Self, &[u8])> {
        match *bytes {
            [dg @ 0x80..=0xBF, drdg_dbdg, ref rest @ ..] => Some((Self::Luma([dg & 0x3F, (drdg_dbdg & 0xF0) >> 4, drdg_dbdg & 0x0F]), rest)),
            _ => None,
        }
    }

    fn from_bytes_rgb(bytes: &[u8]) -> Option<(Self, &[u8])> {
        match *bytes {
            [0xFE, r, g, b, ref rest @ ..] => Some((Self::ColorRGB([r, g, b]), rest)),
            _ => None,
        }
    }

    fn from_bytes_rgba(bytes: &[u8]) -> Option<(Self, &[u8])> {
        match *bytes {
            [0xFF, r, g, b, a, ref rest @ ..] => Some((Self::ColorRGBA([r, g, b, a]), rest)),
            _ => None,
        }
    }

    pub fn to_bytes(self) -> ([u8; 5], usize) {
        match self {
            Self::Index(idx) => ([idx & 0x3F, 0, 0, 0, 0], 1),
            Self::Diff([dr, dg, db]) => ([0x40 | ((dr & 3) << 4) | ((dg & 3) << 2) | (db & 3), 0, 0, 0, 0], 1),
            Self::Luma([dg, drdg, dbdg]) => ([0x80 | (dg & 0x3F), ((drdg & 15) << 4) | (dbdg & 15), 0, 0, 0], 2),
            Self::Run(run_length) => ([0xC0 | (run_length & 0x3F), 0, 0, 0, 0], 1),
            Self::ColorRGB([r, g, b]) => ([0xFE, r, g, b, 0], 4),
            Self::ColorRGBA([r, g, b, a]) => ([0xFF, r, g, b, a], 5),
        }
    }
}
