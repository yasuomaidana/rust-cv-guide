//! The `oxipix` imgproc backend of the benchmark rows: one function per row that runs the kernel on the same
//! input and with the same parameters as the tutorial snippet.

use crate::op_closures;
use crate::spec::{Assets, Ops};
use cvg_snippets::vis::{ConsensusResult, consensus_matches};
use oxipix_next::imgproc::{
    arith, bilateral, blur, canny, distance, equalize, features, filter2d, gradient, hough, median,
    morph, pyramid, quality, template, threshold, warp,
};

/// A row's backend: builds the timed closure and the digest closure from the shared inputs.
type Entry = (&'static str, for<'a> fn(&'a Assets) -> Ops<'a>);

/// Defines one function per row (`name`), so every adapter is small, and the table that finds them by row id.
macro_rules! backends {
    ($($name:ident, $id:literal, |$a:ident| $e:expr;)*) => {
        $( fn $name<'a>($a: &'a Assets) -> Ops<'a> { op_closures!($e) } )*
        const TABLE: &[Entry] = &[ $( ($id, $name), )* ];
    };
}

/// Source positions of the quadrilateral and the page rectangle it is unwarped to.
fn page(a: &Assets) -> [(f32, f32); 4] {
    let (w, h) = (
        a.invoice_photo.width() as f32 - 1.0,
        a.invoice_photo.height() as f32 - 1.0,
    );
    [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)]
}

/// Destination-to-source map of "turn clockwise by `degrees` about the centre and scale".
fn rotation_map(a: &Assets, degrees: f32, scale: f32) -> [f32; 6] {
    let (cx, cy) = (
        a.astronaut.width() as f32 / 2.0,
        a.astronaut.height() as f32 / 2.0,
    );
    let (s, c) = (-degrees.to_radians()).sin_cos();
    let k = 1.0 / scale;
    [
        c * k,
        -s * k,
        cx - (c * cx - s * cy) * k,
        s * k,
        c * k,
        cy - (s * cx + c * cy) * k,
    ]
}

/// Non-max-suppressed FAST-9 corners.
fn fast_corners(image: &image::GrayImage, threshold: u8) -> Vec<features::Corner> {
    features::suppress_non_max(
        &features::fast9(image, threshold),
        image.width(),
        image.height(),
    )
}

/// Descriptors of the `max` strongest corners that leave room for the BRIEF patch, with their positions.
fn describe(image: &image::GrayImage, max: usize) -> (Vec<(f32, f32)>, Vec<features::Descriptor>) {
    let (w, h) = image.dimensions();
    let usable: Vec<_> = fast_corners(image, 20)
        .into_iter()
        .filter(|c| {
            c.x >= features::MARGIN
                && c.y >= features::MARGIN
                && c.x + features::MARGIN <= w
                && c.y + features::MARGIN <= h
        })
        .collect();
    let points: Vec<(u32, u32)> = features::strongest(usable, max)
        .iter()
        .map(|c| (c.x, c.y))
        .collect();
    let descriptors = features::brief(image, &points, &cvg_snippets::brief::pairs());
    (
        points.iter().map(|&(x, y)| (x as f32, y as f32)).collect(),
        descriptors,
    )
}

/// FAST + BRIEF + Hamming matching between the photo and its shifted copy, then the consensus shift.
fn match_shifted(a: &Assets) -> ConsensusResult {
    let ((ka, da), (kb, db)) = (describe(&a.building, 250), describe(&a.building_shift, 250));
    let pairs: Vec<_> = features::match_nearest(&da, &db, 32)
        .into_iter()
        .map(|(i, j, _)| (ka[i], kb[j]))
        .collect();
    consensus_matches(&pairs, 2.0)
}

fn red_ranges() -> [threshold::HsvRange; 2] {
    let range = |hue_low, hue_high| threshold::HsvRange {
        hue_low,
        hue_high,
        saturation: 128,
        value: 102,
    };
    [range(244, 255), range(0, 11)]
}

backends! {
    template_matching, "template_matching", |a| template::match_template(&a.chelsea_blur, &a.tmpl, template::Method::Sse);
    median_blur, "smoothing_median", |a| median::median_blur(&a.chelsea, 2);
    erode, "erosion", |a| morph::erode(&a.coins_mask, 2, 2);
    top_hat, "opening_closing_hats", |a| morph::top_hat(&a.coins, 3, 3);
    pyr_down, "pyramids", |a| pyramid::pyr_down(&pyramid::pyr_down(&pyramid::pyr_down(&a.chelsea)));
    gaussian, "smoothing_gaussian", |a| blur::gaussian_blur(&a.chelsea, 1.1);
    box_blur, "smoothing_box", |a| blur::box_blur(&a.chelsea, 2);
    bilateral_filter, "smoothing_bilateral", |a| bilateral::bilateral_filter(&a.chelsea, 2, 30.0, 2.0);
    filter_2d, "filter_2d", |a| filter2d::filter2d(&a.chelsea, &[1.0 / 25.0; 25], 5);
    laplace, "laplace", |a| gradient::normalize_abs(&gradient::laplacian(&blur::gaussian_blur(&a.building, 1.0)));
    sobel, "sobel", |a| gradient::scale_abs(&gradient::sobel_x(&a.building));
    canny_edges, "canny", |a| canny::canny(&a.building, 50.0, 150.0);
    hough_lines, "hough_lines", |a| hough::hough_lines(&canny::canny(&a.building, 50.0, 150.0), 150);
    hough_circles, "hough_circle", |a| hough::hough_circles(
        &a.coins,
        &hough::CircleParams { min_radius: 15, max_radius: 40, min_distance: 20.0, canny_high: 150.0, votes: 25 },
    );
    add_weighted, "adding_images", |a| arith::add_weighted(&a.linux_logo, &a.win_logo, (0.5, 0.5, 0.0));
    linear_transform, "basic_linear_transform", |a| arith::convert_scale(&a.coffee, 1.3, 40.0);
    otsu, "threshold", |a| threshold::threshold_binary(&a.coins, threshold::otsu_level(&a.coins));
    in_range, "threshold_inRange", |a| threshold::in_range_hsv(&a.win_logo, &red_ranges());
    equalize_hist, "equalize", |a| equalize::equalize_hist(&a.building);
    psnr, "psnr", |a| quality::psnr(&a.building, &a.building_blur);
    ssim, "ssim", |a| quality::ssim(&a.building, &a.building_blur);
    warp_affine, "warp_affine", |a| warp::warp_affine(&a.astronaut, &rotation_map(a, 30.0, 0.6), a.astronaut.dimensions());
    unwarp, "homography", |a| warp::warp_perspective(
        &a.invoice_photo,
        &warp::homography(&page(a), &a.quad).expect("four corners in general position"),
        a.invoice_photo.dimensions(),
    );
    fast, "feature_detection_fast", |a| fast_corners(&a.building, 30);
    oriented, "feature_detection_orb", |a| features::strongest(fast_corners(&a.building, 30), 250);
    description, "feature_description", |a| match_shifted(a);
    euclidean, "distance_transform", |a| distance::euclidean(&a.coins_mask);
}

/// The `oxipix` backend of the row `id`, or none when `oxipix` has no kernel for it yet.
pub fn lookup<'a>(id: &str, assets: &'a Assets) -> Ops<'a> {
    TABLE
        .iter()
        .find(|(name, _)| *name == id)
        .map_or((None, None), |(_, build)| build(assets))
}
