//! OpenCV tutorial `tutorial_hough_lines` in pure Rust: Canny edges, then the standard Hough line transform.
//! Run with `cargo run -p cvg-snippets --example hough_lines`.

use cvg_snippets::common::{Lines, gray_sample};
use image::GrayImage;
use imageproc::{
    edges::canny,
    hough::{LineDetectionOptions, detect_lines},
};

/// Neighbourhood radius (in accumulator buckets) the page starts with: only the strongest line in each 17 x 17 window is kept.
pub const SUPPRESSION: u32 = 8;

/// Lines that at least `votes` edge pixels lie on; only the strongest in each 17 x 17 bucket neighbourhood are kept.
pub fn lines(input: &GrayImage, votes: u32) -> Lines {
    lines_with(input, votes, SUPPRESSION)
}

/// [`lines`] keeping the strongest line in each `(2 * radius + 1)` square of buckets. Radius 1 is what
/// `cv::HoughLines` does, so it returns every local peak of the accumulator.
pub fn lines_with(input: &GrayImage, votes: u32, radius: u32) -> Lines {
    let options = LineDetectionOptions {
        vote_threshold: votes,
        suppression_radius: radius,
    };
    let found = detect_lines(&canny(input, 50.0, 150.0), options);
    found
        .iter()
        .map(|l| (l.r, (l.angle_in_degrees as f32).to_radians()))
        .collect()
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
