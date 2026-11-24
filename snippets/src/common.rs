//! Helpers every example shares, so inputs and golden hashes are made one way.

use image::{DynamicImage, Rgb, RgbImage};
use std::path::Path;

/// A named picture an example produces; `describe` prints a pixel of each.
pub type Figure = (&'static str, DynamicImage);

/// Loads one of the photographs in `snippets/assets/` (see `ATTRIBUTION.md` there for sources and licences).
pub fn sample(name: &str) -> RgbImage {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(name);
    image::open(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
        .to_rgb8()
}

/// `name(x,y) = pixel` for every figure, one per line: colour figures print `[r, g, b]`, grey ones the single level.
pub fn describe(figures: &[Figure], x: u32, y: u32) -> String {
    let lines = figures.iter().map(|(name, picture)| {
        let value = if picture.color().has_color() {
            format!("{:?}", picture.to_rgb8().get_pixel(x, y).0)
        } else {
            picture.to_luma8().get_pixel(x, y).0[0].to_string()
        };
        format!("{name}({x},{y}) = {value}")
    });
    lines.collect::<Vec<_>>().join("\n")
}

/// A deterministic test image: red grows left to right, green top to bottom, blue is an 8-pixel checkerboard.
pub fn gradient(width: u32, height: u32) -> RgbImage {
    RgbImage::from_fn(width, height, |x, y| {
        let red = (x * 255 / width) as u8;
        let green = (y * 255 / height) as u8;
        let blue = if (x / 8 + y / 8) % 2 == 0 { 255 } else { 0 };
        Rgb([red, green, blue])
    })
}

/// 64-bit FNV-1a hash, for golden-output tests.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(b"", 0xcbf2_9ce4_8422_2325)]
    #[case(b"a", 0xaf63_dc4c_8601_ec8c)]
    fn fnv1a_matches_the_reference_values(#[case] input: &[u8], #[case] expected: u64) {
        assert_eq!(fnv1a(input), expected);
    }

    #[rstest]
    #[case(0, 0, [0, 0, 255])]
    #[case(8, 0, [31, 0, 0])]
    #[case(0, 8, [0, 31, 0])]
    fn gradient_pixels(#[case] x: u32, #[case] y: u32, #[case] expected: [u8; 3]) {
        assert_eq!(gradient(64, 64).get_pixel(x, y).0, expected);
    }

    #[test]
    fn describe_prints_colour_and_grey_pixels() {
        let figures = vec![
            ("c", DynamicImage::ImageRgb8(gradient(64, 64))),
            ("g", DynamicImage::ImageRgb8(gradient(64, 64)).grayscale()),
        ];
        let text = describe(&figures, 8, 0);
        assert!(text.starts_with("c(8,0) = [31, 0, 0]\ng(8,0) = "));
    }

    #[test]
    fn samples_load_from_the_assets_folder() {
        assert_eq!(sample("LinuxLogo.jpg").dimensions(), (320, 240));
    }

    #[test]
    #[should_panic(expected = "cannot read")]
    fn a_missing_sample_panics_with_its_path() {
        sample("nope.png");
    }

    #[test]
    fn empty_gradient_has_no_pixels() {
        assert_eq!(gradient(0, 0).dimensions(), (0, 0));
    }
}
