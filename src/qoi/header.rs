use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Channels {
    RGB,
    RGBA,
}

impl TryFrom<u8> for Channels {
    type Error = ();

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::RGB),
            4 => Ok(Self::RGBA),
            _ => Err(()),
        }
    }
}

impl Default for Channels {
    fn default() -> Self { Self::RGBA }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorSpace {
    SRGBLinearAlpha,
    Linear,
}

impl TryFrom<u8> for ColorSpace {
    type Error = ();

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SRGBLinearAlpha),
            1 => Ok(Self::Linear),
            _ => Err(()),
        }
    }
}

impl Default for ColorSpace {
    fn default() -> Self { Self::SRGBLinearAlpha }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Header {
    pub width: u32,
    pub height: u32,
    pub channels: Channels,
    pub color_space: ColorSpace,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            width: 0u32,
            height: 0u32,
            channels: Default::default(),
            color_space: Default::default(),
        }
    }
}

impl Header {
    pub fn new() -> Self { Default::default() }

    pub fn from_bytes(bytes: &[u8]) -> Result<Header> {
        if bytes.len() < 14 {
            return Err(QoIError::new_with_description(ErrorType::IOError, "insufficient bytes for header".into()));
        }

        let magic_bytes = <[u8; 4]>::try_from(&bytes[0..4]).unwrap();
        let width_bytes = <[u8; 4]>::try_from(&bytes[4..8]).unwrap();
        let height_bytes = <[u8; 4]>::try_from(&bytes[8..12]).unwrap();
        let channels_byte = bytes[12];
        let color_space_byte = bytes[13];

        match magic_bytes {
            [0x71, 0x6F, 0x69, 0x66] => (),
            _ => {
                return Err(QoIError::new(ErrorType::BadMagic));
            }
        };

        let header = Header {
            width: u32::from_be_bytes(width_bytes),
            height: u32::from_be_bytes(height_bytes),
            channels: TryFrom::try_from(channels_byte).map_err(|_| QoIError::new(ErrorType::BadMetadata))?,
            color_space: TryFrom::try_from(color_space_byte).map_err(|_| QoIError::new(ErrorType::BadMetadata))?,
        };

        Ok(header)
    }

    pub fn consume_from_bytes(bytes: &[u8]) -> Result<(Header, &[u8])> {
        if bytes.len() < 14 {
            return Err(QoIError::new_with_description(ErrorType::IOError, "insufficient bytes for header".into()));
        }

        Ok((Self::from_bytes(&bytes[..14])?, &bytes[14..]))
    }

    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Header> {
        let mut bytes = [0u8; 14];
        reader.read_exact(&mut bytes)?;

        Self::from_bytes(&bytes)
    }

    pub fn to_bytes(&self) -> [u8; 14] {
        let mut bytes = [0; 14];

        bytes[0..4].iter_mut().zip([0x71, 0x6F, 0x69, 0x66].iter()).for_each(|(d, s)| *d = *s);
        bytes[4..8].iter_mut().zip(self.width.to_be_bytes().iter()).for_each(|(d, s)| *d = *s);
        bytes[8..12].iter_mut().zip(self.height.to_be_bytes().iter()).for_each(|(d, s)| *d = *s);
        bytes[12] = match self.channels {
            Channels::RGB => 3u8,
            Channels::RGBA => 4u8,
        };
        bytes[13] = match self.color_space {
            ColorSpace::SRGBLinearAlpha => 0u8,
            ColorSpace::Linear => 1u8,
        };

        bytes
    }

    pub fn encode_to_vec(&self, vec: &mut Vec<u8>) {
        self.encode_to_writer(vec).unwrap()
    }

    pub fn encode_to_writer<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.to_bytes())
    }
}
