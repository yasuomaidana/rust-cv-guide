//! Benchmark specs for imgproc filter, edge, Hough, and geometry tutorials.

use crate::spec::{Assets, BenchSpec};
use crate::{
    bench_one, bench_pair, canny, filter2d, hough_lines, laplace, oxi_canny, oxi_filter2d,
    oxi_hough_circles, oxi_hough_lines, oxi_laplacian, oxi_pyramids, oxi_smoothing, oxi_sobel,
    oxi_warp_affine, pyramids, smoothing, sobel, template_matching, warp_affine,
};
use imageproc::template_matching::MatchTemplateMethod;

/// Builds benchmark specifications for smoothing, edges, Hough, and geometry tutorials.
pub fn collect(a: &Assets) -> Vec<BenchSpec<'_>> {
    vec![
        bench_pair!(
            "smoothing_box",
            "tutorial_gausian_median_blur_bilateral_filter",
            "imgproc",
            "Box blur (5x5, r=2)",
            "451x300 RGB",
            20,
            "oximedia-cv",
            None,
            smoothing::box_blur(&a.chelsea, 2),
            oxi_smoothing::box_blur(&a.chelsea, 2)
        ),
        bench_pair!(
            "smoothing_gaussian",
            "tutorial_gausian_median_blur_bilateral_filter",
            "imgproc",
            "Gaussian blur (sigma=1.1)",
            "451x300 RGB",
            15,
            "oximedia-cv",
            None,
            smoothing::gaussian(&a.chelsea, 1.1),
            oxi_smoothing::gaussian(&a.chelsea, 1.1)
        ),
        bench_pair!(
            "smoothing_median",
            "tutorial_gausian_median_blur_bilateral_filter",
            "imgproc",
            "Median filter (5x5, r=2)",
            "451x300 RGB",
            10,
            "oximedia-cv",
            Some("OXI-93"),
            smoothing::median(&a.chelsea, 2),
            oxi_smoothing::median(&a.chelsea, 2)
        ),
        bench_pair!(
            "smoothing_bilateral",
            "tutorial_gausian_median_blur_bilateral_filter",
            "imgproc",
            "Bilateral filter (d=5, r=2, sc=30)",
            "451x300 RGB",
            10,
            "oximedia-cv",
            Some("OXI-93"),
            smoothing::bilateral(&a.chelsea, 2, 30.0),
            oxi_smoothing::bilateral(&a.chelsea, 2, 30.0)
        ),
        bench_pair!(
            "filter_2d",
            "tutorial_filter_2d",
            "imgproc",
            "2D convolution (5x5 box kernel)",
            "451x300 RGB",
            15,
            "oximedia-cv",
            None,
            filter2d::filter_2d(&a.chelsea, filter2d::Preset::Box(5)),
            oxi_filter2d::filter_2d(&a.chelsea, 0, 5)
        ),
        bench_pair!(
            "laplace",
            "tutorial_laplace_operator",
            "imgproc",
            "Gaussian blur + Laplacian edge",
            "640x480 grey",
            20,
            "oximedia-cv",
            None,
            laplace::laplacian(&a.building, 1.0),
            oxi_laplacian::laplacian(&a.building, 1.0)
        ),
        bench_pair!(
            "sobel",
            "tutorial_sobel_derivatives",
            "imgproc",
            "Sobel X derivative",
            "640x480 grey",
            20,
            "oximedia-cv",
            None,
            sobel::grad_x(&a.building),
            oxi_sobel::grad_x(&a.building)
        ),
        bench_pair!(
            "canny",
            "tutorial_canny_detector",
            "imgproc",
            "Gaussian blur + Canny (50/150)",
            "640x480 grey",
            15,
            "oximedia-cv",
            None,
            canny::detect_edges(&a.building, 50.0, 150.0),
            oxi_canny::detect_edges(&a.building, 50.0, 150.0)
        ),
        bench_pair!(
            "hough_lines",
            "tutorial_hough_lines",
            "imgproc",
            "Canny + standard Hough lines",
            "640x480 grey",
            10,
            "oximedia-cv",
            None,
            hough_lines::lines(&a.building, hough_lines::VOTES),
            oxi_hough_lines::lines(&a.building, oxi_hough_lines::VOTES)
        ),
        bench_one!(
            oxi: "hough_circle", "tutorial_hough_circle", "imgproc",
            "Canny + circle Hough (r=15..40)", "384x303 grey", 5, "oximedia-cv",
            oxi_hough_circles::circles(&a.coins, oxi_hough_circles::MIN_RADIUS, oxi_hough_circles::MAX_RADIUS, oxi_hough_circles::VOTES)
        ),
        bench_pair!(
            "pyramids",
            "tutorial_pyramids",
            "imgproc",
            "3-level pyrDown",
            "451x300 RGB",
            20,
            "oximedia-cv",
            None,
            pyramids::pyramid(&a.chelsea, 3, false),
            oxi_pyramids::pyramid(&a.chelsea, 3, false)
        ),
        bench_pair!(
            "warp_affine",
            "tutorial_warp_affine",
            "imgproc",
            "Rotate 30 deg + scale 0.6",
            "512x512 RGB",
            20,
            "oximedia-cv",
            None,
            warp_affine::rotate_scale(&a.astronaut, 30.0, 0.6),
            oxi_warp_affine::rotate_scale(&a.astronaut, 30.0, 0.6)
        ),
        bench_one!(
            rust: "template_matching", "tutorial_template_matching", "imgproc",
            "Template match (TM_SQDIFF)", "451x300 vs 40x40 grey", 10, "OXI-92",
            template_matching::best_match(&a.chelsea_blur, &a.tmpl, MatchTemplateMethod::SumOfSquaredErrors)
        ),
    ]
}
