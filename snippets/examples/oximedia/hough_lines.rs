//! OpenCV tutorial `tutorial_hough_lines` with `oximedia-cv` instead of `imageproc`.
//! Run with `cargo run -p cvg-snippets --example oxi_hough_lines`.

use cvg_snippets::common::{Lines, edge_points, gray_sample, suppress_lines};
use image::GrayImage;
use imageproc::edges::canny;
use oximedia_cv::hough_transform::{HoughAccumulator, HoughLineConfig};

/// Whether `(t_idx, r_idx)` has no strictly stronger neighbour in a `17 x 17` accumulator window (wrapping across `theta = PI`).
fn is_peak(acc: &HoughAccumulator, t_idx: usize, r_idx: usize, votes: u32) -> bool {
    let radius = 8isize;
    let n_theta = (acc.num_theta - 1) as isize;
    for dt in -radius..=radius {
        let nt_raw = t_idx as isize + dt;
        let flip = nt_raw < 0 || nt_raw >= n_theta;
        let nt = nt_raw.rem_euclid(n_theta) as usize;
        let centre_r = if flip { acc.num_rho - 1 - r_idx } else { r_idx };
        let lo = centre_r.saturating_sub(radius as usize);
        let hi = (centre_r + radius as usize).min(acc.num_rho - 1);
        if acc.data[nt * acc.num_rho + lo..=nt * acc.num_rho + hi]
            .iter()
            .any(|&v| v > votes)
        {
            return false;
        }
    }
    true
}

/// Lines that at least `votes` Canny edge pixels lie on, using `HoughAccumulator` with 17x17 local-maximum suppression
/// and antipodal wrap-around (excluding the duplicate `theta = PI` row).
pub fn lines(input: &GrayImage, votes: u32) -> Lines {
    let config = HoughLineConfig {
        vote_threshold: votes,
        ..HoughLineConfig::default()
    };
    let mut acc = HoughAccumulator::new(input.width() as usize, input.height() as usize, &config);
    for (x, y) in edge_points(&canny(input, 50.0, 150.0)) {
        acc.vote(x, y);
    }
    let mut peaks = Vec::new();
    for t_idx in 0..acc.num_theta.saturating_sub(1) {
        for r_idx in 0..acc.num_rho {
            let count = acc.data[t_idx * acc.num_rho + r_idx];
            if count >= votes && is_peak(&acc, t_idx, r_idx, count) {
                let rho = (r_idx as f64 * acc.rho_resolution - acc.rho_offset) as f32;
                let theta = (t_idx as f64 * acc.theta_resolution) as f32;
                peaks.push((count, rho, theta));
            }
        }
    }
    peaks.sort_by_key(|a| std::cmp::Reverse(a.0));
    let sorted: Lines = peaks
        .into_iter()
        .map(|(_, rho, theta)| (rho, theta))
        .collect();
    suppress_lines(&sorted, 8.0, 8.0_f32.to_radians())
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
