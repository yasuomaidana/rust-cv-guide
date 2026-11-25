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

/// Blurs away speckle, then keeps what is darker than `level` as 255 on 0: the ink of a scanned page, the mask the contour examples trace.
pub fn ink_mask(input: &GrayImage, level: u8) -> GrayImage {
    let smooth = image::imageops::blur(input, 0.8);
    GrayImage::from_fn(smooth.width(), smooth.height(), |x, y| {
        Luma([if smooth.get_pixel(x, y).0[0] < level {
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

/// The grey picture dimmed, with a small red cross on each point.
pub fn draw_points(base: &GrayImage, points: &[(f32, f32)]) -> RgbImage {
    let mut canvas = RgbImage::from_fn(base.width(), base.height(), |x, y| {
        let Luma([level]) = *base.get_pixel(x, y);
        Rgb([level / 2; 3])
    });
    for (x, y) in points {
        for step in -2_i32..=2 {
            for (px, py) in [(*x as i32 + step, *y as i32), (*x as i32, *y as i32 + step)] {
                if px >= 0 && py >= 0 && (px as u32) < base.width() && (py as u32) < base.height() {
                    canvas.put_pixel(px as u32, py as u32, Rgb([255, 40, 40]));
                }
            }
        }
    }
    canvas
}

/// The grey picture dimmed, with each `(x, y, radius)` circle drawn as a red ring.
pub fn draw_circles(base: &GrayImage, circles: &[(f32, f32, f32)]) -> RgbImage {
    let mut canvas = RgbImage::from_fn(base.width(), base.height(), |x, y| {
        let Luma([level]) = *base.get_pixel(x, y);
        Rgb([level / 2; 3])
    });
    for (cx, cy, radius) in circles {
        let steps = (radius * 8.0).max(16.0) as u32;
        for step in 0..steps {
            let angle = step as f32 / steps as f32 * std::f32::consts::TAU;
            let (x, y) = (cx + radius * angle.cos(), cy + radius * angle.sin());
            if x >= 0.0 && y >= 0.0 && x < base.width() as f32 && y < base.height() as f32 {
                canvas.put_pixel(x as u32, y as u32, Rgb([255, 40, 40]));
            }
        }
    }
    canvas
}

/// The five operations of the morphology tutorial, each on a grey picture with a square window of `2 * radius + 1`.
pub struct Hats {
    pub open: fn(&GrayImage, u8) -> GrayImage,
    pub close: fn(&GrayImage, u8) -> GrayImage,
    pub top_hat: fn(&GrayImage, u8) -> GrayImage,
    pub black_hat: fn(&GrayImage, u8) -> GrayImage,
    pub gradient: fn(&GrayImage, u8) -> GrayImage,
}

/// One operation by number: 0 opening, 1 closing, 2 top hat, 3 black hat, anything else the gradient.
pub fn hat(ops: &Hats, kind: u32, grey: &GrayImage, radius: u8) -> GrayImage {
    let op = match kind {
        0 => ops.open,
        1 => ops.close,
        2 => ops.top_hat,
        3 => ops.black_hat,
        _ => ops.gradient,
    };
    op(grey, radius)
}

/// The coin photograph and the five operations on it with a 7x7 window.
pub fn hat_figures(ops: &Hats) -> Vec<Figure> {
    let grey = gray_sample("coins.png");
    let named = ["opened", "closed", "top_hat", "black_hat", "gradient"];
    let results = named
        .iter()
        .enumerate()
        .map(|(kind, name)| (*name, hat(ops, kind as u32, &grey, 3)));
    let figures = std::iter::once(("input", grey.clone())).chain(results);
    figures
        .map(|(name, picture)| (name, DynamicImage::ImageLuma8(picture)))
        .collect()
}

/// Straight lines in normal form: `(rho, theta)` with `x * cos(theta) + y * sin(theta) = rho`, theta in radians.
pub type Lines = Vec<(f32, f32)>;

/// The grey picture dimmed, with each line drawn across it in red.
pub fn draw_lines(base: &GrayImage, lines: &Lines) -> RgbImage {
    let mut canvas = RgbImage::from_fn(base.width(), base.height(), |x, y| {
        let Luma([level]) = *base.get_pixel(x, y);
        Rgb([level / 2; 3])
    });
    let reach = (base.width() + base.height()) as i32 * 2;
    for (rho, theta) in lines {
        let (sin, cos) = theta.sin_cos();
        for step in -reach..=reach {
            let t = step as f32 * 0.5;
            let (x, y) = (rho * cos - t * sin, rho * sin + t * cos);
            if x >= 0.0 && y >= 0.0 && x < base.width() as f32 && y < base.height() as f32 {
                canvas.put_pixel(x as u32, y as u32, Rgb([255, 40, 40]));
            }
        }
    }
    canvas
}

/// Keeps a line only when no stronger kept line (the input is strongest first) is within `rho_gap` pixels and `theta_gap` radians.
pub fn suppress_lines(strongest_first: &Lines, rho_gap: f32, theta_gap: f32) -> Lines {
    let mut kept: Lines = Vec::new();
    for (rho, theta) in strongest_first {
        let near =
            |(r, t): &(f32, f32)| (rho - r).abs() <= rho_gap && (theta - t).abs() <= theta_gap;
        if !kept.iter().any(near) {
            kept.push((*rho, *theta));
        }
    }
    kept
}

/// The coordinates of the non-zero pixels of an edge map, the points that vote in a Hough transform.
pub fn edge_points(edges: &GrayImage) -> Vec<(f64, f64)> {
    let on = edges.enumerate_pixels().filter(|(_, _, p)| p.0[0] > 0);
    on.map(|(x, y, _)| (f64::from(x), f64::from(y))).collect()
}

/// The picture pasted at the top left of a black `width` x `height` canvas (cropped if larger): a result of another size, shown at the input's.
pub fn on_canvas(width: u32, height: u32, picture: &RgbImage) -> RgbImage {
    RgbImage::from_fn(width, height, |x, y| {
        if x < picture.width() && y < picture.height() {
            *picture.get_pixel(x, y)
        } else {
            Rgb([0, 0, 0])
        }
    })
}

/// The grey picture dimmed, with a red rectangle with its top left corner at `(x, y)`.
pub fn draw_box(base: &GrayImage, (x, y): (u32, u32), (width, height): (u32, u32)) -> RgbImage {
    RgbImage::from_fn(base.width(), base.height(), |px, py| {
        let on_border = (px >= x && px <= x + width && py >= y && py <= y + height)
            && (px - x < 2 || x + width - px < 2 || py - y < 2 || y + height - py < 2);
        if on_border {
            Rgb([255, 40, 40])
        } else {
            Rgb([base.get_pixel(px, py).0[0] / 2; 3])
        }
    })
}

/// What a contour example prints: how many outlines it found and how many points they have in total.
pub fn outline_report(outlines: &Outlines) -> String {
    let points: usize = outlines.iter().map(Vec::len).sum();
    format!("outlines = {}\npoints = {points}", outlines.len())
}

/// Corners of the invoice in `invoice_photo.png` (640 x 480), clockwise from the top left.
pub const INVOICE_QUAD: [(f32, f32); 4] =
    [(70.0, 40.0), (580.0, 70.0), (555.0, 430.0), (40.0, 400.0)];

/// The invoice corners scaled to a `width` x `height` picture and pulled `shrink` (0 to 0.3) of the way to the centre, which crops the page.
pub fn invoice_quad(width: u32, height: u32, shrink: f32) -> [(f32, f32); 4] {
    let (sx, sy) = (width as f32 / 640.0, height as f32 / 480.0);
    let scaled = INVOICE_QUAD.map(|(x, y)| (x * sx, y * sy));
    let centre = scaled
        .iter()
        .fold((0.0, 0.0), |c, p| (c.0 + p.0 / 4.0, c.1 + p.1 / 4.0));
    scaled.map(|(x, y)| {
        (
            centre.0 + (x - centre.0) * (1.0 - shrink),
            centre.1 + (y - centre.1) * (1.0 - shrink),
        )
    })
}

/// The photographed invoice and the page straightened by `unwarp`: the two figures of a homography example.
pub fn unwarp_figures(unwarp: impl Fn(&RgbImage, &[(f32, f32); 4]) -> RgbImage) -> Vec<Figure> {
    let input = sample("invoice_photo.png");
    let page = unwarp(&input, &invoice_quad(input.width(), input.height(), 0.0));
    rgb_figures(input, [("page", page)])
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
    fn points_are_marked_inside_the_picture_only() {
        let base = GrayImage::from_pixel(8, 8, Luma([100]));
        let marked = draw_points(&base, &[(4.0, 4.0), (-5.0, 3.0)]);
        assert_eq!(marked.get_pixel(5, 4).0, [255, 40, 40]);
        assert_eq!(marked.get_pixel(0, 0).0, [50, 50, 50]);
    }

    #[test]
    fn circles_are_drawn_where_they_fit() {
        let base = GrayImage::from_pixel(20, 20, Luma([100]));
        let drawn = draw_circles(&base, &[(10.0, 10.0, 5.0), (-30.0, 0.0, 4.0)]);
        assert_eq!(drawn.get_pixel(15, 10).0, [255, 40, 40]);
        assert_eq!(drawn.get_pixel(10, 10).0, [50, 50, 50]);
    }

    #[test]
    fn hats_are_chosen_by_number() {
        let flat: fn(&GrayImage, u8) -> GrayImage = |image, _| image.clone();
        let bright: fn(&GrayImage, u8) -> GrayImage =
            |image, _| GrayImage::from_pixel(image.width(), image.height(), Luma([255]));
        let ops = Hats {
            open: flat,
            close: bright,
            top_hat: flat,
            black_hat: flat,
            gradient: flat,
        };
        let grey = GrayImage::new(2, 2);
        assert_eq!(hat(&ops, 1, &grey, 1).get_pixel(0, 0).0, [255]);
        assert_eq!(hat(&ops, 9, &grey, 1).get_pixel(0, 0).0, [0]);
        assert_eq!(hat_figures(&ops).len(), 6);
    }

    #[test]
    fn lines_are_drawn_and_thinned() {
        let base = GrayImage::from_pixel(20, 20, Luma([100]));
        let drawn = draw_lines(&base, &vec![(10.0, 0.0)]);
        assert_eq!(drawn.get_pixel(10, 5).0, [255, 40, 40]);
        assert_eq!(drawn.get_pixel(3, 5).0, [50, 50, 50]);
        let thinned = suppress_lines(&vec![(10.0, 0.5), (11.0, 0.52), (40.0, 0.5)], 2.0, 0.1);
        assert_eq!(thinned, vec![(10.0, 0.5), (40.0, 0.5)]);
        let points = edge_points(&GrayImage::from_fn(3, 3, |x, y| {
            Luma([if x == y { 255 } else { 0 }])
        }));
        assert_eq!(points, vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)]);
    }

    #[test]
    fn pictures_are_pasted_or_cropped_onto_a_canvas() {
        let small = RgbImage::from_pixel(2, 2, Rgb([9, 9, 9]));
        let pasted = on_canvas(4, 3, &small);
        assert_eq!(
            (pasted.get_pixel(1, 1).0, pasted.get_pixel(3, 2).0),
            ([9, 9, 9], [0, 0, 0])
        );
        assert_eq!(on_canvas(1, 1, &small).dimensions(), (1, 1));
    }

    #[test]
    fn boxes_are_outlined_in_red() {
        let base = GrayImage::from_pixel(20, 20, Luma([100]));
        let drawn = draw_box(&base, (5, 5), (8, 6));
        assert_eq!(drawn.get_pixel(5, 8).0, [255, 40, 40]);
        assert_eq!(drawn.get_pixel(9, 8).0, [50, 50, 50]);
    }

    #[test]
    fn the_invoice_quad_scales_and_shrinks() {
        assert_eq!(invoice_quad(640, 480, 0.0), INVOICE_QUAD);
        assert_eq!(invoice_quad(320, 240, 0.0)[0], (35.0, 20.0));
        let (x, y) = invoice_quad(640, 480, 0.2)[0];
        assert!(x > 70.0 && y > 40.0);
        assert_eq!(unwarp_figures(|image, _| image.clone()).len(), 2);
        assert_eq!(gray_sample("invoice.png").dimensions(), (560, 420));
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
