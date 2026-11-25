//! OpenCV tutorial `tutorial_hough_lines` with `oximedia-cv` instead of `imageproc`.
//! Run with `cargo run -p cvg-snippets --example oxi_hough_lines`.

use cvg_snippets::common::{Lines, edge_points, gray_sample, suppress_lines};
use image::GrayImage;
use imageproc::edges::canny;
use oximedia_cv::hough_transform::{HoughLineConfig, hough_lines};

/// Lines that at least `votes` edge pixels lie on. `oximedia-cv` returns every accumulator bucket above the threshold,
/// strongest first, so the example keeps only lines that are not within 8 pixels and 8 degrees of a stronger one.
pub fn lines(input: &GrayImage, votes: u32) -> Lines {
    let config = HoughLineConfig {
        vote_threshold: votes,
        max_lines: 2000,
        ..HoughLineConfig::default()
    };
    let points = edge_points(&canny(input, 50.0, 150.0));
    let found = hough_lines(
        &points,
        input.width() as usize,
        input.height() as usize,
        &config,
    );
    let all: Lines = found
        .iter()
        .map(|l| (l.rho as f32, l.theta as f32))
        .collect();
    suppress_lines(&all, 8.0, 8.0_f32.to_radians())
}

/// Votes the page starts with.
pub const VOTES: u32 = 150;

/// How many lines the building photo gives.
pub fn report() -> String {
    format!(
        "lines = {}",
        lines(&gray_sample("building.jpg"), VOTES).len()
    )
}

pub fn main() {
    println!("{}", report());
}
