//! OpenCV tutorial `tutorial_hough_lines` in pure Rust: Canny edges, then the standard Hough line transform.
//! Run with `cargo run -p cvg-snippets --example hough_lines`.

use cvg_snippets::common::{Lines, gray_sample};
use image::GrayImage;
use imageproc::{
    edges::canny,
    hough::{LineDetectionOptions, detect_lines},
};

/// Lines that at least `votes` edge pixels lie on; only the strongest in each 17 x 17 bucket neighbourhood are kept.
pub fn lines(input: &GrayImage, votes: u32) -> Lines {
    let options = LineDetectionOptions {
        vote_threshold: votes,
        suppression_radius: 8,
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
