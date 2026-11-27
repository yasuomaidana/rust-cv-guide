//! Timing statistics, output fingerprints (digests), parity checks and probe calibration.

use image::{ImageBuffer, Pixel};
use serde::Serialize;
use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

/// Spread of one timing series, in milliseconds.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Stats {
    pub median: f64,
    pub p10: f64,
    pub p90: f64,
    /// Median absolute deviation.
    pub mad: f64,
    pub iters: usize,
}

impl Stats {
    /// Relative noise floor, `2 * MAD / median`; a gap smaller than this is not a real difference.
    #[cfg(test)]
    pub fn noise(&self) -> f64 {
        if self.median > 0.0 { 2.0 * self.mad / self.median } else { 0.0 }
    }
}

/// Cheap output fingerprint that both languages can compute: size plus mean sample value.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, serde::Deserialize)]
pub struct Digest {
    pub w: u32,
    pub h: u32,
    pub mean: f64,
}

/// Wraps a value so [`ImageDigest`] / [`NoDigest`] pick the image fingerprint when one exists.
pub struct Probe<'a, T>(pub &'a T);

/// Fingerprint for 8-bit image buffers.
pub trait ImageDigest {
    fn digest(&self) -> Option<Digest>;
}

impl<P: Pixel<Subpixel = u8>> ImageDigest for Probe<'_, ImageBuffer<P, Vec<u8>>> {
    fn digest(&self) -> Option<Digest> {
        let raw = self.0.as_raw();
        let sum: u64 = raw.iter().map(|&v| u64::from(v)).sum();
        Some(Digest {
            w: self.0.width(),
            h: self.0.height(),
            mean: sum as f64 / raw.len().max(1) as f64,
        })
    }
}

/// Keypoint and match lists: only the count is comparable (`h == 1`).
impl<T> ImageDigest for Probe<'_, Vec<T>> {
    fn digest(&self) -> Option<Digest> {
        Some(Digest { w: self.0.len() as u32, h: 1, mean: 0.0 })
    }
}

/// Fallback for outputs without a comparable image (counts, locations, float maps).
pub trait NoDigest {
    fn digest(&self) -> Option<Digest>;
}

impl<T> NoDigest for &Probe<'_, T> {
    fn digest(&self) -> Option<Digest> {
        None
    }
}

/// Builds the timed closure and the one-off digest closure for the same expression.
#[macro_export]
macro_rules! op_closures {
    ($e:expr) => {
        (
            Some(Box::new(move || {
                std::hint::black_box($e);
            }) as Box<dyn FnMut() + '_>),
            Some(Box::new(move || {
                #[allow(unused_imports)]
                use $crate::measure::{ImageDigest, NoDigest};
                let out = $e;
                (&$crate::measure::Probe(&out)).digest()
            }) as Box<dyn FnMut() -> Option<$crate::measure::Digest> + '_>),
        )
    };
    () => {
        (None, None)
    };
}

/// Minimum total measured time per series, so fast ops get enough samples for a stable spread.
const MIN_TOTAL_MS: f64 = 200.0;
const MAX_ITERS: usize = 5000;

/// Warm-up, then adaptive-length timing: at least `iters` samples and at least [`MIN_TOTAL_MS`] in total.
pub fn time_stats(iters: usize, mut f: impl FnMut()) -> Stats {
    let mut run = || {
        let start = Instant::now();
        f();
        black_box(());
        start.elapsed().as_secs_f64() * 1000.0
    };
    let first = (0..3).map(|_| run()).fold(0.0, f64::max);
    let wanted = ((MIN_TOTAL_MS / first.max(1e-6)) as usize).clamp(iters.max(1), MAX_ITERS);
    let mut samples: Vec<f64> = (0..wanted).map(|_| run()).collect();
    samples.sort_by(f64::total_cmp);
    let median = samples[samples.len() / 2];
    let mut dev: Vec<f64> = samples.iter().map(|v| (v - median).abs()).collect();
    dev.sort_by(f64::total_cmp);
    let at = |q: f64| samples[((samples.len() - 1) as f64 * q) as usize];
    Stats {
        median: round3(median),
        p10: round3(at(0.1)),
        p90: round3(at(0.9)),
        mad: round3(dev[dev.len() / 2]),
        iters: wanted,
    }
}

/// Median of an adaptive timing series, in milliseconds.
pub fn time_ms(iters: usize, f: impl FnMut()) -> f64 {
    time_stats(iters, f).median
}

/// Time for one allocate-and-drop of a `w x h x c` byte buffer; the probe-overhead floor for that input size.
pub fn alloc_floor(w: usize, h: usize, c: usize) -> f64 {
    time_ms(100, || {
        black_box(vec![0u8; w * h * c]);
    })
}

/// Probe-overhead floor: an empty closure and the allocation cost of each input size used by the rows.
pub fn calibration() -> BTreeMap<String, f64> {
    let mut map = BTreeMap::new();
    map.insert("empty_closure".into(), time_ms(100, || {}));
    for (name, w, h, c) in [
        ("alloc_320x240x3", 320, 240, 3),
        ("alloc_451x300x3", 451, 300, 3),
        ("alloc_640x480x1", 640, 480, 1),
        ("alloc_640x480x3", 640, 480, 3),
    ] {
        map.insert(name.into(), alloc_floor(w, h, c));
    }
    map
}

/// True when two fingerprints describe the same work: counts (`h == 1`) within 25 %, images by size and
/// mean within 2 grey levels or 5 %.
fn digests_agree(x: Digest, y: Digest) -> bool {
    if x.h == 1 && y.h == 1 {
        return (f64::from(x.w) - f64::from(y.w)).abs() <= 0.25 * f64::from(x.w.max(y.w)).max(1.0);
    }
    x.w == y.w && x.h == y.h && (x.mean - y.mean).abs() <= 2.0f64.max(0.05 * x.mean.abs())
}

/// `ok` or `mismatch` when both fingerprints exist, `n/a` otherwise.
pub fn parity(a: Option<Digest>, b: Option<Digest>) -> &'static str {
    match (a, b) {
        (Some(x), Some(y)) if digests_agree(x, y) => "ok",
        (Some(_), Some(_)) => "mismatch",
        _ => "n/a",
    }
}


/// Folds per-backend parity verdicts: any `mismatch` wins, then any `ok`, else `n/a`.
pub fn combine_parity(checks: [&'static str; 2]) -> &'static str {
    ["mismatch", "ok"]
        .into_iter()
        .find(|verdict| checks.contains(verdict))
        .unwrap_or("n/a")
}

/// Rounds a millisecond measurement to 3 decimal places.
pub fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}
