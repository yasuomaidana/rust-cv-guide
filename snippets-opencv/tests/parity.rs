//! Each `opencv` crate example must agree with its pure-Rust twin on the guide's photographs.
//!
//! These tests need a native OpenCV, so they run only in this standalone crate (`cargo test` here).

#![allow(dead_code)]

#[path = "../../snippets/examples/core/adding_images.rs"]
mod rust_adding;
#[path = "../../snippets/examples/imgproc_edges/canny.rs"]
mod rust_canny;
#[path = "../../snippets/examples/imgproc_color/equalize.rs"]
mod rust_equalize;
#[path = "../../snippets/examples/imgproc_morph/erosion.rs"]
mod rust_erosion;
#[path = "../../snippets/examples/imgproc_filters/laplace.rs"]
mod rust_laplace;
#[path = "../../snippets/examples/core/basic_linear_transform.rs"]
mod rust_linear;
#[path = "../../snippets/examples/imgproc_geom/pyramids.rs"]
mod rust_pyramids;
#[path = "../../snippets/examples/imgproc_filters/smoothing.rs"]
mod rust_smoothing;
#[path = "../../snippets/examples/imgproc_filters/sobel.rs"]
mod rust_sobel;
#[path = "../../snippets/examples/imgproc_geom/template_matching.rs"]
mod rust_template;
#[path = "../../snippets/examples/imgproc_color/threshold.rs"]
mod rust_threshold;
#[path = "../../snippets/examples/imgproc_geom/warp_affine.rs"]
mod rust_warp;

#[path = "../../snippets/examples/opencv_core/adding_images.rs"]
mod ocv_adding;
#[path = "../../snippets/examples/opencv_filters/canny.rs"]
mod ocv_canny;
#[path = "../../snippets/examples/opencv_color/equalize.rs"]
mod ocv_equalize;
#[path = "../../snippets/examples/opencv_morph/erosion.rs"]
mod ocv_erosion;
#[path = "../../snippets/examples/opencv_filters/laplace.rs"]
mod ocv_laplace;
#[path = "../../snippets/examples/opencv_core/basic_linear_transform.rs"]
mod ocv_linear;
#[path = "../../snippets/examples/opencv_geom/pyramids.rs"]
mod ocv_pyramids;
#[path = "../../snippets/examples/opencv_filters/smoothing.rs"]
mod ocv_smoothing;
#[path = "../../snippets/examples/opencv_filters/sobel.rs"]
mod ocv_sobel;
#[path = "../../snippets/examples/opencv_geom/template_matching.rs"]
mod ocv_template;
#[path = "../../snippets/examples/opencv_color/threshold.rs"]
mod ocv_threshold;
#[path = "../../snippets/examples/opencv_geom/warp_affine.rs"]
mod ocv_warp;

use cvg_snippets::common::{gray_sample, sample};
use image::{GrayImage, RgbImage};
use imageproc::contrast::ThresholdType;
use imageproc::template_matching::MatchTemplateMethod;

/// Mean absolute difference between two pictures of the same size (grey levels).
fn mad(a: &[u8], b: &[u8]) -> f64 {
    assert_eq!(a.len(), b.len(), "the pictures must have the same size");
    a.iter()
        .zip(b)
        .map(|(x, y)| f64::from(x.abs_diff(*y)))
        .sum::<f64>()
        / a.len() as f64
}

/// Pearson correlation of two pictures of the same size.
fn correlation(a: &[u8], b: &[u8]) -> f64 {
    let n = a.len() as f64;
    let (ma, mb) = (
        a.iter().map(|v| f64::from(*v)).sum::<f64>() / n,
        b.iter().map(|v| f64::from(*v)).sum::<f64>() / n,
    );
    let (mut cov, mut va, mut vb) = (0.0, 0.0, 0.0);
    for (x, y) in a.iter().zip(b) {
        let (dx, dy) = (f64::from(*x) - ma, f64::from(*y) - mb);
        (cov, va, vb) = (cov + dx * dy, va + dx * dx, vb + dy * dy);
    }
    cov / (va * vb).sqrt()
}

fn rgb_mad(a: &RgbImage, b: &RgbImage) -> f64 {
    assert_eq!(a.dimensions(), b.dimensions());
    mad(a.as_raw(), b.as_raw())
}

fn grey_mad(a: &GrayImage, b: &GrayImage) -> f64 {
    assert_eq!(a.dimensions(), b.dimensions());
    mad(a.as_raw(), b.as_raw())
}

#[test]
fn smoothing_agrees() {
    let photo = sample("chelsea.png");
    assert!(
        rgb_mad(
            &rust_smoothing::box_blur(&photo, 2),
            &ocv_smoothing::box_blur(&photo, 2)
        ) < 1.0
    );
    assert!(
        rgb_mad(
            &rust_smoothing::gaussian(&photo, 1.1),
            &ocv_smoothing::gaussian(&photo, 1.1)
        ) < 2.0
    );
    assert!(
        rgb_mad(
            &rust_smoothing::median(&photo, 2),
            &ocv_smoothing::median(&photo, 2)
        ) < 1.0
    );
    assert!(
        rgb_mad(
            &rust_smoothing::bilateral(&photo, 2, 30.0),
            &ocv_smoothing::bilateral(&photo, 2, 30.0)
        ) < 6.0
    );
}

#[test]
fn canny_finds_a_similar_number_of_edges() {
    let photo = gray_sample("building.jpg");
    let count = |image: &GrayImage| image.iter().filter(|v| **v > 0).count() as f64;
    let ratio = count(&ocv_canny::detect_edges(&photo, 50.0, 150.0))
        / count(&rust_canny::detect_edges(&photo, 50.0, 150.0));
    assert!((0.7..1.4).contains(&ratio), "edge count ratio {ratio}");
}

#[test]
fn derivatives_agree() {
    let photo = gray_sample("building.jpg");
    assert!(grey_mad(&rust_sobel::grad_x(&photo), &ocv_sobel::grad_x(&photo)) < 3.0);
    // Both scale by the single strongest response, so absolute levels depend on one pixel; compare the shape.
    let (rust, ocv) = (
        rust_laplace::laplacian(&photo, 1.0),
        ocv_laplace::laplacian(&photo, 1.0),
    );
    let similarity = correlation(rust.as_raw(), ocv.as_raw());
    assert!(similarity > 0.97, "laplacian correlation {similarity}");
}

#[test]
fn geometry_agrees() {
    let photo = sample("astronaut.png");
    assert!(
        rgb_mad(
            &rust_warp::rotate_scale(&photo, 30.0, 0.6),
            &ocv_warp::rotate_scale(&photo, 30.0, 0.6)
        ) < 8.0
    );
    let chelsea = sample("chelsea.png");
    assert!(
        rgb_mad(
            &rust_pyramids::pyramid(&chelsea, 3, false),
            &ocv_pyramids::pyramid(&chelsea, 3, false)
        ) < 3.0
    );
}

#[test]
fn template_matching_finds_the_same_place() {
    let photo = gray_sample("chelsea.png");
    let patch = image::imageops::crop_imm(&photo, 100, 80, 40, 40).to_image();
    let theirs = rust_template::best_match(&photo, &patch, MatchTemplateMethod::SumOfSquaredErrors);
    assert_eq!(ocv_template::best_match(&photo, &patch), theirs);
    assert_eq!(theirs, (100, 80));
}

#[test]
fn colour_and_core_operations_agree() {
    let coins = gray_sample("coins.png");
    let differing = |a: &GrayImage, b: &GrayImage| {
        a.iter().zip(b.iter()).filter(|(x, y)| x != y).count() as f64 / a.as_raw().len() as f64
    };
    let (mine, theirs) = (
        ocv_threshold::binarize(&coins, None),
        rust_threshold::binarize(&coins, None, ThresholdType::Binary),
    );
    assert!(differing(&mine, &theirs) < 0.01);
    let building = gray_sample("building.jpg");
    assert!(
        grey_mad(
            &rust_equalize::equalize(&building),
            &ocv_equalize::equalize(&building)
        ) < 3.0
    );
    let (a, b) = (sample("LinuxLogo.jpg"), sample("WindowsLogo.jpg"));
    let (ours, theirs) = (
        ocv_adding::add_weighted(&a, &b, 0.5, 0.5).unwrap(),
        rust_adding::add_weighted(&a, &b, 0.5, 0.5).unwrap(),
    );
    assert!(rgb_mad(&ours, &theirs) < 1.1);
    let coffee = sample("coffee.png");
    assert!(
        rgb_mad(
            &rust_linear::linear_transform(&coffee, 1.3, 40.0),
            &ocv_linear::linear_transform(&coffee, 1.3, 40.0)
        ) < 1.1
    );
    let mask = rust_threshold::binarize(&coins, None, ThresholdType::Binary);
    assert!(
        differing(
            &rust_erosion::shrink(&mask, 2),
            &ocv_erosion::shrink(&mask, 2)
        ) < 0.01
    );
}
