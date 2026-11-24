//! Helpers every example shares, so inputs and golden hashes are made one way.

use image::{Rgb, RgbImage};

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
    fn empty_gradient_has_no_pixels() {
        assert_eq!(gradient(0, 0).dimensions(), (0, 0));
    }
}
