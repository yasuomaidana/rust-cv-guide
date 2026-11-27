//! BRIEF sampling and FAST non-max suppression shared by every backend of the feature tutorials, so
//! `imageproc`, `oximedia-cv` and OpenCV all run the same pipeline: FAST-9 corners with 3x3 non-max
//! suppression, then 256 intensity tests inside a 31x31 patch.
//!
//! The sampler reproduces `imageproc::binary_descriptors::brief` exactly (5x5 box means at the test
//! points, bit set when the first mean is smaller), so descriptors from both crates are comparable bit
//! for bit; `oximedia-cv` has no BRIEF that takes a test pattern.

use image::GrayImage;
use std::collections::HashMap;

/// Keypoints must be this many pixels inside the picture for the 31x31 patch to fit.
pub const MARGIN: u32 = 17;
/// Descriptor length in bits.
pub const BITS: usize = 256;
const PATCH: u32 = 31;
const PATCH_RADIUS: u32 = 15;
const MEAN_RADIUS: u32 = 2;

/// One BRIEF test: two points `(x0, y0, x1, y1)` inside the 31x31 patch.
pub type Pair = (u32, u32, u32, u32);

/// A deterministic set of 256 test pairs, so results repeat across runs, crates and WebAssembly.
pub fn pairs() -> Vec<Pair> {
    let mut state = 0x5EED_CAFE_1234_5678_u64;
    let mut coordinate = || {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((state >> 32) % u64::from(PATCH)) as u32
    };
    (0..BITS)
        .map(|_| (coordinate(), coordinate(), coordinate(), coordinate()))
        .collect()
}

/// Running sums that give the mean of any box in constant time.
pub struct Integral {
    width: usize,
    height: usize,
    sums: Vec<u32>,
}

impl Integral {
    pub fn new(image: &GrayImage) -> Self {
        let (width, height) = (image.width() as usize, image.height() as usize);
        let stride = width + 1;
        let mut sums = vec![0u32; stride * (height + 1)];
        for y in 0..height {
            let mut row = 0u32;
            for x in 0..width {
                row += u32::from(image.as_raw()[y * width + x]);
                sums[(y + 1) * stride + x + 1] = sums[y * stride + x + 1] + row;
            }
        }
        Self {
            width,
            height,
            sums,
        }
    }

    fn at(&self, x: usize, y: usize) -> u32 {
        self.sums[y * (self.width + 1) + x]
    }

    /// Mean (rounded down) of the `(2r + 1)` square box centred on `(x, y)`, clipped to the picture.
    pub fn mean(&self, x: u32, y: u32, r: u32) -> u8 {
        let x0 = x.saturating_sub(r) as usize;
        let y0 = y.saturating_sub(r) as usize;
        let x1 = ((x + r + 1) as usize).min(self.width);
        let y1 = ((y + r + 1) as usize).min(self.height);
        if x1 <= x0 || y1 <= y0 {
            return 0;
        }
        let total = self.at(x1, y1) + self.at(x0, y0) - self.at(x1, y0) - self.at(x0, y1);
        (total / ((x1 - x0) * (y1 - y0)) as u32) as u8
    }
}

/// The descriptor bits of the keypoint at `(x, y)`; the keypoint must be at least [`MARGIN`] from every edge.
pub fn bits(integral: &Integral, x: u32, y: u32, pairs: &[Pair]) -> Vec<bool> {
    let corner = (x - PATCH_RADIUS, y - PATCH_RADIUS);
    let mean = |px: u32, py: u32| integral.mean(px + corner.0 + 1, py + corner.1 + 1, MEAN_RADIUS);
    pairs
        .iter()
        .map(|&(x0, y0, x1, y1)| mean(x0, y0) < mean(x1, y1))
        .collect()
}

/// Keeps the corners that beat all eight neighbours (equal scores resolved by position), as OpenCV's
/// `FastFeatureDetector(nonmaxSuppression=true)` does. `corners` are `(x, y, score)`.
pub fn suppress_non_max(corners: &[(u32, u32, f32)]) -> Vec<(u32, u32, f32)> {
    let scores: HashMap<(u32, u32), f32> = corners.iter().map(|&(x, y, s)| ((x, y), s)).collect();
    let beats_all = |&(x, y, s): &(u32, u32, f32)| {
        (-1i64..=1)
            .flat_map(|dy| (-1i64..=1).map(move |dx| (dx, dy)))
            .filter(|&d| d != (0, 0))
            .filter_map(|(dx, dy)| {
                Some((
                    x.checked_add_signed(dx as i32)?,
                    y.checked_add_signed(dy as i32)?,
                ))
            })
            .all(|at| {
                scores
                    .get(&at)
                    .is_none_or(|&o| o < s || (o == s && at > (x, y)))
            })
    };
    corners.iter().copied().filter(beats_all).collect()
}

/// Whether a keypoint at `(x, y)` leaves room for the patch in a `width` x `height` picture.
pub fn fits(x: u32, y: u32, width: u32, height: u32) -> bool {
    x >= MARGIN && x + MARGIN <= width && y >= MARGIN && y + MARGIN <= height
}
