use super::*;

pub struct Image {
    pub header: Header,
    pub data: Box<[Color]>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            header: Header {
                width,
                height,
                channels: Channels::RGBA,
                color_space: ColorSpace::SRGBLinearAlpha,
            },
            data: vec![Color::default(); height as usize * width as usize].into_boxed_slice(),
        }
    }

    pub fn parse_from_qoi_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 14 + 8 {
            return Err(QoIError::new_with_description(ErrorType::InsufficientData, format!("not enough data, expected at least 14, got {}", bytes.len())));
        }

        let header_bytes = &bytes[0..14];
        let mut data_bytes = &bytes[14..bytes.len() - 8];
        let end_mark_bytes = &bytes[bytes.len() - 8..];

        let header = Header::from_bytes(header_bytes)?;

        let mut colors = vec![Color::default(); header.height as usize * header.width as usize].into_boxed_slice();

        let mut decoder = Decoder::new(&mut colors);

        while !data_bytes.is_empty() {
            let (command, rest_of_the_bytes) = RawCommand::from_bytes(data_bytes)
                .map(|v| Ok(v))
                .unwrap_or(Err(QoIError::new_with_description(ErrorType::InsufficientData, "insufficient bytes for a full command".into())))?;

            decoder.process(command)?;

            data_bytes = rest_of_the_bytes;
        }

        Ok(Self { header, data: colors })
    }

    pub fn from_pixels_it<It: Iterator<Item = Color>>(width: u32, height: u32, it: It) -> Self {
        let mut ret = Self::new(width, height);

        let len = ret.len();
        for (target, source) in std::iter::zip(ret.data.iter_mut(), it.take(len)) {
            *target = source;
        }

        ret
    }

    pub fn encode_to_writer<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.header.encode_to_writer(writer)?;

        let mut writer_fn = |buf: &[u8]| -> std::io::Result<()> { writer.write_all(buf) };
        let mut encoder = Encoder::new(&mut writer_fn);
        for c in self.data.iter() {
            encoder.feed(*c)?;
        }
        encoder.finish()?;

        let end_mark = [0, 0, 0, 0, 0, 0, 0, 1];
        writer.write_all(&end_mark)?;

        Ok(())
    }

    pub fn encode_to_vec(&self, vec: &mut Vec<u8>) { self.encode_to_writer(vec).unwrap() }

    pub fn len(&self) -> usize { self.data.len() }
    pub fn height(&self) -> u32 { self.header.height }
    pub fn width(&self) -> u32 { self.header.width }

    pub fn pixel(&self, col: usize, row: usize) -> &Color { &self.data[col + row * self.width() as usize] }
    pub fn pixel_mut(&mut self, col: usize, row: usize) -> &mut Color { &mut self.data[col + row * self.width() as usize] }
}
