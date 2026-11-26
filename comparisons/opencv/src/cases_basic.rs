//! Benchmark specs for core, app, features, color, and morphology tutorials.

use crate::spec::{Assets, BenchSpec};
use crate::{
    adding_images, basic_linear_transform, bench_one, bench_pair, contours, distance, equalize,
    erosion, fast, hats, homography, in_range, oxi_adding_images, oxi_equalize, oxi_erosion,
    oxi_in_range, oxi_morphology, oxi_orb, oxi_quality, oxipix_contours, oxipix_homography,
    oxipix_threshold, psnr, threshold,
};
use cvg_snippets::common::FeatureAlgorithm;
use imageproc::contrast::ThresholdType;

fn feature_spec<'a>(
    a: &'a Assets,
    id: &'static str,
    op: &'static str,
    iters: usize,
    alg: FeatureAlgorithm,
) -> BenchSpec<'a> {
    bench_pair!(
        id,
        "tutorial_feature_detection",
        "features",
        op,
        "640x480 grey",
        iters,
        "oximedia-cv",
        None,
        fast::detect(&a.building, alg, 30, 250),
        oxi_orb::detect(&a.building, alg, 30, 250)
    )
}

/// Builds benchmark specifications for core, app, features, color, and morph tutorials.
pub fn collect(a: &Assets) -> Vec<BenchSpec<'_>> {
    vec![
        bench_pair!(
            "adding_images",
            "tutorial_adding_images",
            "core",
            "addWeighted (0.5 / 0.5)",
            "320x240 RGB",
            30,
            "oximedia-cv",
            None,
            adding_images::add_weighted(&a.linux_logo, &a.win_logo, 0.5, 0.5),
            oxi_adding_images::add_weighted(&a.linux_logo, &a.win_logo, 0.5)
        ),
        bench_one!(
            rust: "basic_linear_transform", "tutorial_basic_linear_transform", "core",
            "Contrast + brightness (a=1.3, b=40)", "600x400 RGB", 30, "OXI-88",
            basic_linear_transform::linear_transform(&a.coffee, 1.3, 40.0)
        ),
        bench_pair!(
            "psnr",
            "tutorial_video_input_psnr_ssim",
            "app",
            "PSNR",
            "640x480 grey",
            30,
            "oximedia-cv",
            None,
            psnr::psnr(&a.building, &a.building_blur),
            oxi_quality::psnr(&a.building, &a.building_blur)
        ),
        bench_pair!(
            "ssim",
            "tutorial_video_input_psnr_ssim",
            "app",
            "SSIM (11x11 Gaussian)",
            "640x480 grey",
            15,
            "oximedia-cv",
            None,
            psnr::ssim(&a.building, &a.building_blur),
            oxi_quality::ssim(&a.building, &a.building_blur)
        ),
        feature_spec(
            a,
            "feature_detection_fast",
            "FAST-9 keypoints (t=30, max=250)",
            20,
            FeatureAlgorithm::Fast,
        ),
        feature_spec(
            a,
            "feature_detection_orb",
            "ORB keypoints (t=30, max=250)",
            10,
            FeatureAlgorithm::Orb,
        ),
        bench_pair!(
            "homography",
            "tutorial_homography",
            "features",
            "Perspective unwarp (4-point quad)",
            "640x480 RGB",
            20,
            "oxipix",
            None,
            homography::unwarp(&a.invoice_photo, &a.quad),
            oxipix_homography::unwarp(&a.invoice_photo, &a.quad)
        ),
        bench_pair!(
            "threshold",
            "tutorial_threshold",
            "imgproc",
            "Otsu binary threshold",
            "384x303 grey",
            30,
            "oxipix",
            Some("OXI-89"),
            threshold::binarize(&a.coins, None, ThresholdType::Binary),
            oxipix_threshold::binarize(&a.coins, None, oxipix_threshold::Mode::Binary)
        ),
        bench_pair!(
            "threshold_inRange",
            "tutorial_threshold_inRange",
            "imgproc",
            "HSV red range mask",
            "320x240 RGB",
            30,
            "oximedia-cv",
            Some("OXI-90"),
            in_range::in_range(&a.win_logo, in_range::RED),
            oxi_in_range::in_range(&a.win_logo, oxi_in_range::RED)
        ),
        bench_pair!(
            "equalize",
            "tutorial_histogram_equalization",
            "imgproc",
            "Histogram equalisation",
            "640x480 grey",
            30,
            "oximedia-cv",
            None,
            equalize::equalize(&a.building),
            oxi_equalize::equalize(&a.building)
        ),
        bench_pair!(
            "erosion",
            "tutorial_erosion_dilatation",
            "imgproc",
            "Erode (5x5 square, r=2)",
            "384x303 grey",
            20,
            "oximedia-cv",
            None,
            erosion::shrink(&a.coins_mask, 2),
            oxi_erosion::shrink(&a.coins_mask, 2)
        ),
        bench_pair!(
            "opening_closing_hats",
            "tutorial_opening_closing_hats",
            "imgproc",
            "Top-hat morphology (7x7, r=3)",
            "384x303 grey",
            15,
            "oximedia-cv",
            None,
            hats::top_hat(&a.coins, 3),
            oxi_morphology::top_hat(&a.coins, 3)
        ),
        bench_one!(
            rust: "distance_transform", "tutorial_distance_transform", "imgproc",
            "Distance transform (L2)", "384x303 mask", 20, "OXI-91",
            distance::distances(&a.coins_mask, 1)
        ),
        bench_pair!(
            "find_contours",
            "tutorial_find_contours",
            "imgproc",
            "Find contours + approxPolyDP",
            "560x420 mask",
            15,
            "oxipix",
            None,
            contours::outlines(&a.invoice_mask, 2.0),
            oxipix_contours::outlines(&a.invoice_mask, 2.0)
        ),
    ]
}
