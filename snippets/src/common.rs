//! Helpers every example shares, so inputs and golden hashes are made one way.

use image::{DynamicImage, GrayImage, Luma, Rgb, RgbImage};
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

/// The photo as the first figure, then each named result: the figures of a colour filter example.
pub fn rgb_figures(
    input: RgbImage,
    results: impl IntoIterator<Item = (&'static str, RgbImage)>,
) -> Vec<Figure> {
    std::iter::once(("input", input))
        .chain(results)
        .map(|(name, picture)| (name, DynamicImage::ImageRgb8(picture)))
        .collect()
}

/// The four smoothing filters an example offers, in the order the page lists them.
pub struct Smoothers {
    pub box_blur: fn(&RgbImage, u32) -> RgbImage,
    pub gaussian: fn(&RgbImage, f32) -> RgbImage,
    pub median: fn(&RgbImage, u32) -> RgbImage,
    pub bilateral: fn(&RgbImage, u32, f32) -> RgbImage,
}

/// The photo and four 5x5 smoothings of it, whichever crate does the smoothing.
pub fn smoothing_figures(filters: &Smoothers) -> Vec<Figure> {
    let input = sample("chelsea.png");
    let smoothed = [
        ("box", (filters.box_blur)(&input, 2)),
        ("gaussian", (filters.gaussian)(&input, 1.1)),
        ("median", (filters.median)(&input, 2)),
        ("bilateral", (filters.bilateral)(&input, 2, 30.0)),
    ];
    rgb_figures(input.clone(), smoothed)
}

/// The grey photo and what `detect` makes of it: the two figures of an edge example.
pub fn edge_figures(photo: &str, detect: impl Fn(&GrayImage) -> GrayImage) -> Vec<Figure> {
    let input = DynamicImage::ImageRgb8(sample(photo)).to_luma8();
    let edges = detect(&input);
    vec![
        ("input", DynamicImage::ImageLuma8(input)),
        ("edges", DynamicImage::ImageLuma8(edges)),
    ]
}

/// A sample photograph as a grey image.
pub fn gray_sample(name: &str) -> GrayImage {
    DynamicImage::ImageRgb8(sample(name)).to_luma8()
}

/// Blurs away texture, then keeps what is brighter than `level` as 255 on 0: the mask the contour examples trace.
pub fn bright_mask(input: &GrayImage, level: u8) -> GrayImage {
    let smooth = image::imageops::blur(input, 3.0);
    GrayImage::from_fn(smooth.width(), smooth.height(), |x, y| {
        Luma([if smooth.get_pixel(x, y).0[0] > level {
            255
        } else {
            0
        }])
    })
}

/// Closed outlines as pixel coordinates, as both contour examples produce them.
pub type Outlines = Vec<Vec<(f32, f32)>>;

/// The grey picture dimmed, with each outline drawn on it in red.
pub fn draw_outlines(base: &GrayImage, outlines: &Outlines) -> RgbImage {
    let mut canvas = RgbImage::from_fn(base.width(), base.height(), |x, y| {
        let Luma([level]) = *base.get_pixel(x, y);
        Rgb([level / 3; 3])
    });
    for outline in outlines {
        for (from, to) in outline.iter().zip(outline.iter().cycle().skip(1)) {
            let steps = (to.0 - from.0)
                .abs()
                .max((to.1 - from.1).abs())
                .ceil()
                .max(1.0);
            for step in 0..=steps as u32 {
                let t = step as f32 / steps;
                let (x, y) = (from.0 + (to.0 - from.0) * t, from.1 + (to.1 - from.1) * t);
                if x >= 0.0 && y >= 0.0 && x < base.width() as f32 && y < base.height() as f32 {
                    canvas.put_pixel(x as u32, y as u32, Rgb([255, 40, 40]));
                }
            }
        }
    }
    canvas
}

/// What a contour example prints: how many outlines it found and how many points they have in total.
pub fn outline_report(outlines: &Outlines) -> String {
    let points: usize = outlines.iter().map(Vec::len).sum();
    format!("outlines = {}\npoints = {points}", outlines.len())
}

/// Corners of a quadrilateral inside a `width` x `height` picture, clockwise from the top left;
/// `skew` (0 to 0.4) pulls them in by different amounts so the quad looks like a tilted page.
pub fn skewed_quad(width: u32, height: u32, skew: f32) -> [(f32, f32); 4] {
    let (w, h) = (width as f32, height as f32);
    [
        (skew * w, 0.5 * skew * h),
        (w - 0.5 * skew * w, skew * h),
        (w - skew * w, h - 0.5 * skew * h),
        (0.5 * skew * w, h - skew * h),
    ]
}

/// The photo and the tilted quad of it straightened by `unwarp`: the two figures of a homography example.
pub fn unwarp_figures(unwarp: impl Fn(&RgbImage, &[(f32, f32); 4]) -> RgbImage) -> Vec<Figure> {
    let input = sample("building.jpg");
    let straight = unwarp(&input, &skewed_quad(input.width(), input.height(), 0.15));
    rgb_figures(input, [("unwarped", straight)])
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

    #[test]
    fn outlines_are_drawn_and_counted() {
        let base = GrayImage::from_pixel(8, 8, Luma([90]));
        let square = vec![(1.0, 1.0), (6.0, 1.0), (6.0, 6.0), (1.0, 6.0)];
        let outside = vec![(-3.0, 2.0), (20.0, 2.0)];
        let drawn = draw_outlines(&base, &vec![square.clone(), outside]);
        assert_eq!(drawn.get_pixel(3, 1).0, [255, 40, 40]);
        assert_eq!(drawn.get_pixel(3, 3).0, [30, 30, 30]);
        assert_eq!(outline_report(&vec![square]), "outlines = 1\npoints = 4");
    }

    #[test]
    fn the_quad_stays_inside_the_picture() {
        let quad = skewed_quad(400, 300, 0.15);
        assert!(
            quad.iter()
                .all(|(x, y)| (0.0..=400.0).contains(x) && (0.0..=300.0).contains(y))
        );
        let kept = unwarp_figures(|image, _| image.clone());
        assert_eq!(kept.len(), 2);
        assert_eq!(gray_sample("building.jpg").dimensions(), (868, 600));
    }

    #[test]
    fn figures_are_named_in_order() {
        let names = |figures: Vec<Figure>| figures.iter().map(|f| f.0).collect::<Vec<_>>();
        let results = [("blur", gradient(4, 4))];
        assert_eq!(
            names(rgb_figures(gradient(4, 4), results)),
            ["input", "blur"]
        );
        let identity = Smoothers {
            box_blur: |i, _| i.clone(),
            gaussian: |i, _| i.clone(),
            median: |i, _| i.clone(),
            bilateral: |i, _, _| i.clone(),
        };
        assert_eq!(names(smoothing_figures(&identity)).len(), 5);
        assert_eq!(
            names(edge_figures("building.jpg", GrayImage::clone)),
            ["input", "edges"]
        );
    }

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
