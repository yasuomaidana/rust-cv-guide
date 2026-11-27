//! Benchmark specification, assets fixture, timing loop, and JSON output schema.

use cvg_snippets::common::{gray_sample, ink_mask, invoice_quad, sample};
use image::{GrayImage, ImageBuffer, Pixel, RgbImage, imageops};
use serde::Serialize;
use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

/// Environment metadata recorded alongside benchmark measurements.
#[derive(Debug, Serialize)]
pub struct Environment {
    pub os: String,
    pub arch: String,
    pub opencv_version: String,
    pub rust_profile: String,
    pub oximedia_cv_version: String,
    pub oxipix_version: String,
    pub imageproc_version: String,
    /// Rust builds measured, e.g. `release`, `release+lto+native`.
    pub builds: Vec<String>,
    /// HAL / SIMD / threading lines from `cv::getBuildInformation()`.
    pub opencv_build: Vec<String>,
}

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

/// One measured comparison row across OpenCV, pure Rust, and `oximedia-cv`/`oxipix`.
#[derive(Debug, Serialize)]
pub struct BenchRow {
    pub id: String,
    pub tutorial: String,
    pub section: String,
    pub operation: String,
    pub input: String,
    pub opencv_ms: Option<f64>,
    pub rust_ms: Option<f64>,
    pub oximedia_ms: Option<f64>,
    pub oximedia_crate: Option<String>,
    pub oximedia_vs_rust: Option<f64>,
    pub rust_vs_opencv: Option<f64>,
    pub oximedia_vs_opencv: Option<f64>,
    pub oxipix_ticket: Option<String>,
    pub warning_note: Option<String>,
    pub opencv_fresh_ms: Option<f64>,
    pub rust_opt_ms: Option<f64>,
    pub oximedia_opt_ms: Option<f64>,
    pub rust_stats: Option<Stats>,
    pub oximedia_stats: Option<Stats>,
    pub opencv_digest: Option<Digest>,
    pub rust_digest: Option<Digest>,
    pub oximedia_digest: Option<Digest>,
    /// `ok`, `mismatch`, or `n/a` when a side has no comparable image output.
    pub parity: String,
}

/// Complete serialized benchmark catalog written to `catalog/benchmarks.json`.
#[derive(Debug, Serialize)]
pub struct BenchmarkCatalog {
    pub warn_ratio_threshold: f64,
    pub opencv_warn_ratio_threshold: f64,
    pub environment: Environment,
    /// Probe-overhead floor: empty closure and allocate-and-drop per input size, in milliseconds.
    pub calibration: BTreeMap<String, f64>,
    pub benchmarks: Vec<BenchRow>,
}

/// Pre-decoded inputs shared across benchmark runs so decode cost is excluded.
pub struct Assets {
    pub linux_logo: RgbImage,
    pub win_logo: RgbImage,
    pub coffee: RgbImage,
    pub chelsea: RgbImage,
    pub chelsea_blur: GrayImage,
    pub tmpl: GrayImage,
    pub coins: GrayImage,
    pub coins_mask: GrayImage,
    pub building: GrayImage,
    pub building_blur: GrayImage,
    pub astronaut: RgbImage,
    pub invoice_mask: GrayImage,
    pub invoice_photo: RgbImage,
    pub box_img: GrayImage,
    pub box_in_scene: GrayImage,
    pub quad: [(f32, f32); 4],
}

impl Assets {
    /// Loads and prepares every sample asset once before timing begins.
    pub fn load() -> Self {
        let chelsea = sample("chelsea.png");
        let chelsea_gray = gray_sample("chelsea.png");
        let coins = gray_sample("coins.png");
        let coins_mask = imageproc::contrast::threshold(
            &coins,
            imageproc::contrast::otsu_level(&coins),
            imageproc::contrast::ThresholdType::Binary,
        );
        let building = gray_sample("building.jpg");
        let building_blur = imageops::blur(&building, 2.0);
        let chelsea_blur = imageops::brighten(&imageops::blur(&chelsea_gray, 1.5), 25);
        let tmpl = imageops::crop_imm(
            &chelsea_gray,
            (chelsea_gray.width() - 40) / 2,
            (chelsea_gray.height() * 2 / 5).saturating_sub(20),
            40,
            40,
        )
        .to_image();
        let invoice_photo = sample("invoice_photo.png");
        let quad = invoice_quad(invoice_photo.width(), invoice_photo.height(), 0.0);
        Self {
            linux_logo: sample("LinuxLogo.jpg"),
            win_logo: sample("WindowsLogo.jpg"),
            coffee: sample("coffee.png"),
            chelsea,
            chelsea_blur,
            tmpl,
            coins,
            coins_mask,
            building,
            building_blur,
            astronaut: sample("astronaut.png"),
            invoice_mask: ink_mask(&gray_sample("invoice.png"), 128),
            invoice_photo,
            box_img: gray_sample("box.png"),
            box_in_scene: gray_sample("box_in_scene.png"),
            quad,
        }
    }
}

/// Closure + metadata describing one comparable tutorial operation.
pub struct BenchSpec<'a> {
    pub id: &'static str,
    pub tutorial: &'static str,
    pub section: &'static str,
    pub operation: &'static str,
    pub input: &'static str,
    pub iters: usize,
    pub oximedia_crate: Option<&'static str>,
    pub oxipix_ticket: Option<&'static str>,
    pub rust_op: Option<Box<dyn FnMut() + 'a>>,
    pub oxi_op: Option<Box<dyn FnMut() + 'a>>,
    pub rust_digest: Option<Box<dyn FnMut() -> Option<Digest> + 'a>>,
    pub oxi_digest: Option<Box<dyn FnMut() -> Option<Digest> + 'a>>,
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
                use $crate::spec::{ImageDigest, NoDigest};
                let out = $e;
                (&$crate::spec::Probe(&out)).digest()
            }) as Box<dyn FnMut() -> Option<$crate::spec::Digest> + '_>),
        )
    };
    () => {
        (None, None)
    };
}

/// Declares a two-backend (`rust` vs `oximedia-cv`/`oxipix`) benchmark specification without boilerplate.
#[macro_export]
macro_rules! bench_pair {
    ($id:expr, $tut:expr, $sec:expr, $op:expr, $inp:expr, $n:expr, $cr:expr, $tk:expr, $r:expr, $o:expr) => {
        $crate::bench_spec!($id, $tut, $sec, $op, $inp, $n, Some($cr), $tk, $crate::op_closures!($r), $crate::op_closures!($o))
    };
}

/// Declares a single-backend benchmark specification (when pure Rust or `oximedia`/`oxipix` lacks the op).
#[macro_export]
macro_rules! bench_one {
    (rust: $id:expr, $tut:expr, $sec:expr, $op:expr, $inp:expr, $n:expr, $tk:expr, $r:expr) => {
        $crate::bench_spec!($id, $tut, $sec, $op, $inp, $n, None, Some($tk), $crate::op_closures!($r), $crate::op_closures!())
    };
    (oxi: $id:expr, $tut:expr, $sec:expr, $op:expr, $inp:expr, $n:expr, $cr:expr, $o:expr) => {
        $crate::bench_spec!($id, $tut, $sec, $op, $inp, $n, Some($cr), None, $crate::op_closures!(), $crate::op_closures!($o))
    };
}

/// Assembles a `BenchSpec` from metadata and the two `(timed, digest)` closure pairs.
#[macro_export]
macro_rules! bench_spec {
    ($id:expr, $tut:expr, $sec:expr, $op:expr, $inp:expr, $n:expr, $cr:expr, $tk:expr, $rust:expr, $oxi:expr) => {{
        let (rust_op, rust_digest) = $rust;
        let (oxi_op, oxi_digest) = $oxi;
        $crate::spec::BenchSpec {
            id: $id,
            tutorial: $tut,
            section: $sec,
            operation: $op,
            input: $inp,
            iters: $n,
            oximedia_crate: $cr,
            oxipix_ticket: $tk,
            rust_op,
            oxi_op,
            rust_digest,
            oxi_digest,
        }
    }};
}

/// Rounds a millisecond measurement to 3 decimal places.
pub fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

/// Computes `num / den` rounded to 2 decimal places when both are positive.
pub fn ratio(num: Option<f64>, den: Option<f64>) -> Option<f64> {
    match (num, den) {
        (Some(n), Some(d)) if d > 0.0 => Some((n / d * 100.0).round() / 100.0),
        _ => None,
    }
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

/// OpenCV side of a benchmark JSON: timings, fresh-allocation timings, digests and build information.
pub struct OpenCvData<'a> {
    pub results: &'a BTreeMap<String, f64>,
    pub digests: &'a BTreeMap<String, Digest>,
}

/// Runs a `BenchSpec` once for digests, times both backends, and joins the OpenCV timings and digest.
pub fn evaluate(spec: BenchSpec<'_>, ocv: &OpenCvData<'_>) -> BenchRow {
    let opencv_ms = ocv.results.get(spec.id).copied().map(round3);
    let opencv_fresh_ms = ocv.results.get(&format!("{}__fresh", spec.id)).copied().map(round3);
    let opencv_digest = ocv.digests.get(spec.id).copied();
    let rust_digest = spec.rust_digest.and_then(|mut d| d());
    let oximedia_digest = spec.oxi_digest.and_then(|mut d| d());
    let rust_stats = spec.rust_op.map(|op| time_stats(spec.iters, op));
    let oximedia_stats = spec.oxi_op.map(|op| time_stats(spec.iters, op));
    let rust_ms = rust_stats.map(|s| s.median);
    let oximedia_ms = oximedia_stats.map(|s| s.median);
    let checks = [parity(opencv_digest, rust_digest), parity(opencv_digest, oximedia_digest)];
    let parity = if checks.contains(&"mismatch") {
        "mismatch"
    } else if checks.contains(&"ok") {
        "ok"
    } else {
        "n/a"
    };
    BenchRow {
        id: spec.id.into(),
        tutorial: spec.tutorial.into(),
        section: spec.section.into(),
        operation: spec.operation.into(),
        input: spec.input.into(),
        opencv_ms,
        rust_ms,
        oximedia_ms,
        oximedia_crate: spec.oximedia_crate.map(str::to_string),
        oximedia_vs_rust: ratio(oximedia_ms, rust_ms),
        rust_vs_opencv: ratio(rust_ms, opencv_ms),
        oximedia_vs_opencv: ratio(oximedia_ms, opencv_ms),
        oxipix_ticket: spec.oxipix_ticket.map(str::to_string),
        warning_note: None,
        opencv_fresh_ms,
        rust_opt_ms: None,
        oximedia_opt_ms: None,
        rust_stats,
        oximedia_stats,
        opencv_digest,
        rust_digest,
        oximedia_digest,
        parity: parity.into(),
    }
}
