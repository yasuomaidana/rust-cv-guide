//! Helpers every example shares, so inputs and golden hashes are made one way.

use image::{DynamicImage, Rgb, RgbImage};
use std::path::{Path, PathBuf};

/// A named output picture an example produces; the page shows it as `/img/examples/<example>/<name>.png`.
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

/// Where the site serves the figures of `example`: `website/static/img/examples/<example>/`.
pub fn figure_dir(example: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../website/static/img/examples");
    root.join(example)
}

/// Writes every figure as `<dir>/<name>.png`, creating `dir` when needed.
pub fn save_figures_to(dir: &Path, figures: &[Figure]) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    for (name, picture) in figures {
        picture
            .save(dir.join(format!("{name}.png")))
            .map_err(std::io::Error::other)?;
    }
    Ok(())
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

/// The test every example shares: its pictures equal the committed PNGs, then `main` runs (and rewrites the same bytes).
pub fn check_example(example: &str, figures: &[Figure], main: fn()) {
    assert!(
        figures_are_committed(example, figures),
        "figures of {example} differ from the committed PNGs"
    );
    main();
}

/// Writes the figures of `example` where the site serves them.
pub fn save_figures(example: &str, figures: &[Figure]) {
    save_figures_to(&figure_dir(example), figures).expect("figures are written");
}

/// True when every figure equals the PNG already in `dir`: the pictures on the page are what the code produces.
pub fn figures_match(dir: &Path, figures: &[Figure]) -> bool {
    figures.iter().all(|(name, picture)| {
        image::open(dir.join(format!("{name}.png")))
            .is_ok_and(|saved| saved.to_rgba8() == picture.to_rgba8())
    })
}

/// [`figures_match`] for the committed figures of `example`.
pub fn figures_are_committed(example: &str, figures: &[Figure]) -> bool {
    figures_match(&figure_dir(example), figures)
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

    fn scratch(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!("cvg-common-{tag}-{}", std::process::id()))
    }

    #[test]
    fn figures_round_trip_and_mismatches_are_detected() {
        let dir = scratch("figures");
        let figures = vec![("a", DynamicImage::ImageRgb8(gradient(8, 8)))];
        save_figures_to(&dir, &figures).unwrap();
        assert!(figures_match(&dir, &figures));
        let other = vec![("a", DynamicImage::ImageRgb8(gradient(8, 16)))];
        assert!(!figures_match(&dir, &other));
        let missing = vec![("b", DynamicImage::ImageRgb8(gradient(8, 8)))];
        assert!(!figures_match(&dir, &missing));
        std::fs::remove_dir_all(dir).unwrap();
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
    fn saving_into_a_file_path_fails() {
        let blocker = scratch("blocker");
        std::fs::write(&blocker, b"x").unwrap();
        let figures = vec![("a", DynamicImage::ImageRgb8(gradient(8, 8)))];
        assert!(save_figures_to(&blocker, &figures).is_err());
        assert!(save_figures_to(&blocker.join("sub"), &figures).is_err());
        std::fs::remove_file(blocker).unwrap();
    }

    #[test]
    fn a_figure_that_cannot_be_encoded_is_an_error() {
        let dir = scratch("encode");
        let figures = vec![("a", DynamicImage::ImageRgb32F(Default::default()))];
        assert!(save_figures_to(&dir, &figures).is_err());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn samples_load_and_figures_live_under_the_site() {
        assert_eq!(sample("LinuxLogo.jpg").dimensions(), (320, 240));
        assert!(figure_dir("x").ends_with("website/static/img/examples/x"));
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
