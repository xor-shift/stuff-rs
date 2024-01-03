mod color;
mod command;
mod decoder;
mod encoder;
mod error;
mod header;
mod image;

pub use color::*;
pub use command::*;
pub use decoder::*;
pub use encoder::*;
pub use error::*;
pub use header::*;
pub use image::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sample_headers() {
        let pairs: &'static [([u8; 14], Header)] = &[(
            [0x71, 0x6F, 0x69, 0x66, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0],
            Header {
                width: 0,
                height: 0,
                channels: Channels::RGB,
                color_space: ColorSpace::SRGBLinearAlpha,
            },
        )];

        for (test_index, (bytes, expected_header)) in pairs.iter().enumerate() {
            let res = Header::from_bytes(&bytes[..]);

            assert!(res.is_ok(), "test at index {}", test_index);
            assert_eq!(res.unwrap(), *expected_header, "test at index {}", test_index);
        }
    }

    fn assert_parse_result(bytes: &[u8], expected: RawCommand) {
        let res = RawCommand::from_bytes(bytes);

        assert!(res.is_some());

        let res = res.unwrap();

        assert_eq!(res.0, expected);
        assert!(res.1.is_empty());
    }

    #[test]
    pub fn test_commands() {
        assert_parse_result(&[0x00], RawCommand::Index(0));
        assert_parse_result(&[0x01], RawCommand::Index(1));
        assert_parse_result(&[0x3F], RawCommand::Index(0x3F));
        assert_parse_result(&[0x40], RawCommand::Diff([0, 0, 0]));
        assert_parse_result(&[0x41], RawCommand::Diff([0, 0, 1]));
        assert_parse_result(&[0x44], RawCommand::Diff([0, 1, 0]));
        assert_parse_result(&[0x50], RawCommand::Diff([1, 0, 0]));
        assert_parse_result(&[0x7F], RawCommand::Diff([3, 3, 3]));
        assert_parse_result(&[0x80, 0x00], RawCommand::Luma([0, 0, 0]));
        assert_parse_result(&[0x80, 0x01], RawCommand::Luma([0, 0, 1]));
        assert_parse_result(&[0x80, 0x10], RawCommand::Luma([0, 1, 0]));
        assert_parse_result(&[0x81, 0x00], RawCommand::Luma([1, 0, 0]));
        assert_parse_result(&[0xBF, 0xFF], RawCommand::Luma([63, 15, 15]));
        assert_parse_result(&[0xC0], RawCommand::Run(0));
        assert_parse_result(&[0xC1], RawCommand::Run(1));
        assert_parse_result(&[0xFD], RawCommand::Run(61));
        assert_parse_result(&[0xFE, 0, 0, 0], RawCommand::ColorRGB([0, 0, 0]));
        assert_parse_result(&[0xFF, 0, 0, 0, 0], RawCommand::ColorRGBA([0, 0, 0, 0]));
    }

    fn test_single_sample(bytes: &[u8], expected_header: Header, expected_image: &[u8]) {
        let header_res = Header::consume_from_bytes(bytes);

        assert!(header_res.is_ok());
        assert_eq!(header_res.unwrap().0, expected_header);

        let image_res = Image::parse_from_qoi_bytes(bytes);
        assert!(image_res.is_ok());

        let image = image_res.unwrap();

        let expected_image = Image::from_pixels_it(
            expected_header.width,
            expected_header.height,
            expected_image.chunks_exact(4).map(|chunk| Color {
                r: chunk[0],
                g: chunk[1],
                b: chunk[2],
                a: chunk[3],
            }),
        );

        assert!(std::iter::zip(image.data.iter(), expected_image.data.iter()).map(|(a, b)| *a == *b).all(|v| v));
    }

    fn test_roundtrip(bytes: &[u8]) {
        let expected_image = Image::parse_from_qoi_bytes(bytes);
        assert!(expected_image.is_ok());
        let expected_image = expected_image.unwrap();

        let mut encoded_image = Vec::new();
        expected_image.encode_to_vec(&mut encoded_image);
        let decoded_image = Image::parse_from_qoi_bytes(&encoded_image);
        assert!(decoded_image.is_ok());
        let decoded_image = decoded_image.unwrap();

        assert_eq!(encoded_image.len(), bytes.len());

        for (i, (color_expected, color_decoded)) in expected_image.data.iter().zip(decoded_image.data.iter()).enumerate() {
            assert_eq!(*color_expected, *color_decoded, "at index {}", i);
        }
    }

    macro_rules! quick_test {
        ($base_name:literal, $width:literal, $height:literal, $channels:path, $cspace:path) => {
            let qoi_bytes = include_bytes!(concat!($base_name, ".qoi"));
            let raw_bytes = include_bytes!(concat!($base_name, ".bin"));

            test_single_sample(
                qoi_bytes,
                Header {
                    width: $width,
                    height: $height,
                    channels: $channels,
                    color_space: $cspace,
                },
                raw_bytes,
            );

            test_roundtrip(qoi_bytes);
        };
    }

    #[test]
    pub fn test_can_parse_samples() {
        quick_test!("./test/qoi_test_images/dice", 800, 600, Channels::RGBA, ColorSpace::SRGBLinearAlpha);
        quick_test!("./test/qoi_test_images/edgecase", 256, 64, Channels::RGBA, ColorSpace::SRGBLinearAlpha);
        quick_test!("./test/qoi_test_images/kodim10", 512, 768, Channels::RGB, ColorSpace::SRGBLinearAlpha);
        quick_test!("./test/qoi_test_images/kodim23", 768, 512, Channels::RGB, ColorSpace::SRGBLinearAlpha);
        quick_test!("./test/qoi_test_images/qoi_logo", 448, 220, Channels::RGBA, ColorSpace::SRGBLinearAlpha);
        quick_test!("./test/qoi_test_images/testcard_rgba", 256, 256, Channels::RGBA, ColorSpace::SRGBLinearAlpha);
        quick_test!("./test/qoi_test_images/testcard", 256, 256, Channels::RGBA, ColorSpace::SRGBLinearAlpha);
        quick_test!("./test/qoi_test_images/wikipedia_008", 1152, 858, Channels::RGB, ColorSpace::SRGBLinearAlpha);
    }
}
