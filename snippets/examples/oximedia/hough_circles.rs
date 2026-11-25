//! OpenCV tutorial `tutorial_hough_circle` with `oximedia-cv`: the circle Hough transform on Canny edges.
//! Run with `cargo run -p cvg-snippets --example oxi_hough_circles`.

use cvg_snippets::common::gray_sample;
use image::GrayImage;
use imageproc::edges::canny;
use oximedia_cv::hough_transform::{HoughCircleConfig, hough_circles};

/// Circles with a radius from `min_radius` to `max_radius` that at least `votes` edge points lie on, as `(x, y, radius)`.
pub fn circles(
    input: &GrayImage,
    min_radius: f32,
    max_radius: f32,
    votes: u32,
) -> Vec<(f32, f32, f32)> {
    let edge_points: Vec<(f64, f64)> = canny(input, 50.0, 150.0)
        .enumerate_pixels()
        .filter(|(_, _, p)| p.0[0] > 0)
        .map(|(x, y, _)| (f64::from(x), f64::from(y)))
        .collect();
    let config = HoughCircleConfig {
        min_radius: f64::from(min_radius),
        max_radius: f64::from(max_radius),
        vote_threshold: votes,
        ..HoughCircleConfig::default()
    };
    let found = hough_circles(
        &edge_points,
        input.width() as usize,
        input.height() as usize,
        &config,
    );
    found
        .iter()
        .map(|c| (c.cx as f32, c.cy as f32, c.radius as f32))
        .collect()
}

/// Settings the page starts with.
pub const MIN_RADIUS: f32 = 15.0;
pub const MAX_RADIUS: f32 = 40.0;
pub const VOTES: u32 = 25;

/// How many coins the Hough transform finds in the coin photograph.
pub fn report() -> String {
    let found = circles(&gray_sample("coins.png"), MIN_RADIUS, MAX_RADIUS, VOTES);
    format!("circles = {}", found.len())
}

pub fn main() {
    println!("{}", report());
}
