//! Benchmark specification, assets fixture, timing loop, and JSON output schema.

use crate::measure::{Digest, Stats, combine_parity, parity, round3, time_stats};
use cvg_snippets::common::{gray_sample, ink_mask, invoice_quad, sample};
use image::{GrayImage, RgbImage, imageops};
use serde::Serialize;
use std::collections::BTreeMap;

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
    /// Where the `oxipix_next` backend comes from.
    pub oxipix_next_source: String,
    /// Rust builds measured, e.g. `release`, `release+lto+native`.
    pub builds: Vec<String>,
    /// HAL / SIMD / threading lines from `cv::getBuildInformation()`.
    pub opencv_build: Vec<String>,
    /// Threads OpenCV uses by default on this machine (the single-thread rows pin it to 1).
    pub opencv_threads: usize,
    /// Whether OpenCV found an OpenCL device it could use.
    pub opencv_opencl: bool,
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
    /// OpenCV with its default number of threads (the single-thread `opencv_ms` is the algorithm comparison).
    pub opencv_mt_ms: Option<f64>,
    pub rust_opt_ms: Option<f64>,
    pub oximedia_opt_ms: Option<f64>,
    pub rust_stats: Option<Stats>,
    pub oximedia_stats: Option<Stats>,
    pub opencv_digest: Option<Digest>,
    pub rust_digest: Option<Digest>,
    pub oximedia_digest: Option<Digest>,
    /// `oxipix` imgproc kernels (git branch): time, optimised-build time, spread, fingerprint and gap to OpenCV.
    pub oxipix_next_ms: Option<f64>,
    pub oxipix_next_opt_ms: Option<f64>,
    pub oxipix_next_stats: Option<Stats>,
    pub oxipix_next_digest: Option<Digest>,
    pub oxipix_next_vs_opencv: Option<f64>,
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
    /// Small, medium and big pictures for the kernels where size and threads matter.
    pub size_sweep: Vec<BenchRow>,
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
    /// The building photo cropped by (12, 7): the second view of the feature-matching benchmark.
    pub building_shift: GrayImage,
    pub astronaut: RgbImage,
    pub invoice_mask: GrayImage,
    pub invoice_photo: RgbImage,
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
            building_shift: cvg_snippets::vis::shifted_view(&building, 12, 7, 0),
            building,
            building_blur,
            astronaut: sample("astronaut.png"),
            invoice_mask: ink_mask(&gray_sample("invoice.png"), 128),
            invoice_photo,
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
    pub next_op: Option<Box<dyn FnMut() + 'a>>,
    pub next_digest: Option<Box<dyn FnMut() -> Option<Digest> + 'a>>,
}

/// The timed closure and the one-off digest closure of a backend.
pub type Ops<'a> = (
    Option<Box<dyn FnMut() + 'a>>,
    Option<Box<dyn FnMut() -> Option<Digest> + 'a>>,
);

impl<'a> BenchSpec<'a> {
    /// Adds the `oxipix` imgproc backend.
    pub fn with_next(mut self, (op, digest): Ops<'a>) -> Self {
        self.next_op = op;
        self.next_digest = digest;
        self
    }
}

/// Declares a two-backend (`rust` vs `oximedia-cv`/`oxipix`) benchmark specification without boilerplate.
#[macro_export]
macro_rules! bench_pair {
    ($id:expr, $tut:expr, $sec:expr, $op:expr, $inp:expr, $n:expr, $cr:expr, $tk:expr, $r:expr, $o:expr) => {
        $crate::bench_spec!(
            $id,
            $tut,
            $sec,
            $op,
            $inp,
            $n,
            Some($cr),
            $tk,
            $crate::op_closures!($r),
            $crate::op_closures!($o)
        )
    };
}

/// Declares a single-backend benchmark specification (when pure Rust or `oximedia`/`oxipix` lacks the op).
#[macro_export]
macro_rules! bench_one {
    (rust: $id:expr, $tut:expr, $sec:expr, $op:expr, $inp:expr, $n:expr, $tk:expr, $r:expr) => {
        $crate::bench_spec!(
            $id,
            $tut,
            $sec,
            $op,
            $inp,
            $n,
            None,
            Some($tk),
            $crate::op_closures!($r),
            $crate::op_closures!()
        )
    };
    (oxi: $id:expr, $tut:expr, $sec:expr, $op:expr, $inp:expr, $n:expr, $cr:expr, $o:expr) => {
        $crate::bench_spec!(
            $id,
            $tut,
            $sec,
            $op,
            $inp,
            $n,
            Some($cr),
            None,
            $crate::op_closures!(),
            $crate::op_closures!($o)
        )
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
            next_op: None,
            next_digest: None,
        }
    }};
}

/// Computes `num / den` rounded to 2 decimal places when both are positive.
pub fn ratio(num: Option<f64>, den: Option<f64>) -> Option<f64> {
    match (num, den) {
        (Some(n), Some(d)) if d > 0.0 => Some((n / d * 100.0).round() / 100.0),
        _ => None,
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
    let opencv_fresh_ms = ocv
        .results
        .get(&format!("{}__fresh", spec.id))
        .copied()
        .map(round3);
    let opencv_mt_ms = ocv
        .results
        .get(&format!("{}__mt", spec.id))
        .copied()
        .map(round3);
    let opencv_digest = ocv.digests.get(spec.id).copied();
    let rust_digest = spec.rust_digest.and_then(|mut d| d());
    let oximedia_digest = spec.oxi_digest.and_then(|mut d| d());
    let next_digest = spec.next_digest.and_then(|mut d| d());
    let rust_stats = spec.rust_op.map(|op| time_stats(spec.iters, op));
    let oximedia_stats = spec.oxi_op.map(|op| time_stats(spec.iters, op));
    let next_stats = spec.next_op.map(|op| time_stats(spec.iters, op));
    let next_ms = next_stats.map(|s| s.median);
    let rust_ms = rust_stats.map(|s| s.median);
    let oximedia_ms = oximedia_stats.map(|s| s.median);
    let parity = combine_parity([
        parity(opencv_digest, rust_digest),
        parity(opencv_digest, oximedia_digest),
        parity(opencv_digest, next_digest),
    ]);
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
        opencv_mt_ms,
        rust_opt_ms: None,
        oximedia_opt_ms: None,
        rust_stats,
        oximedia_stats,
        opencv_digest,
        rust_digest,
        oximedia_digest,
        oxipix_next_ms: next_ms,
        oxipix_next_opt_ms: None,
        oxipix_next_stats: next_stats,
        oxipix_next_digest: next_digest,
        oxipix_next_vs_opencv: ratio(next_ms, opencv_ms),
        parity: parity.into(),
    }
}
