//! Benchmark specification, assets fixture, timing loop, and JSON output schema.

use cvg_snippets::common::{gray_sample, ink_mask, invoice_quad, sample};
use image::{GrayImage, RgbImage, imageops};
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
}

/// Complete serialized benchmark catalog written to `catalog/benchmarks.json`.
#[derive(Debug, Serialize)]
pub struct BenchmarkCatalog {
    pub warn_ratio_threshold: f64,
    pub opencv_warn_ratio_threshold: f64,
    pub environment: Environment,
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
}

/// Declares a two-backend (`rust` vs `oximedia-cv`/`oxipix`) benchmark specification without boilerplate.
#[macro_export]
macro_rules! bench_pair {
    ($id:expr, $tut:expr, $sec:expr, $op:expr, $inp:expr, $n:expr, $cr:expr, $tk:expr, $r:expr, $o:expr) => {
        BenchSpec {
            id: $id,
            tutorial: $tut,
            section: $sec,
            operation: $op,
            input: $inp,
            iters: $n,
            oximedia_crate: Some($cr),
            oxipix_ticket: $tk,
            rust_op: Some(Box::new(move || {
                std::hint::black_box($r);
            })),
            oxi_op: Some(Box::new(move || {
                std::hint::black_box($o);
            })),
        }
    };
}

/// Declares a single-backend benchmark specification (when pure Rust or `oximedia`/`oxipix` lacks the op).
#[macro_export]
macro_rules! bench_one {
    (rust: $id:expr, $tut:expr, $sec:expr, $op:expr, $inp:expr, $n:expr, $tk:expr, $r:expr) => {
        BenchSpec {
            id: $id,
            tutorial: $tut,
            section: $sec,
            operation: $op,
            input: $inp,
            iters: $n,
            oximedia_crate: None,
            oxipix_ticket: Some($tk),
            rust_op: Some(Box::new(move || {
                std::hint::black_box($r);
            })),
            oxi_op: None,
        }
    };
    (oxi: $id:expr, $tut:expr, $sec:expr, $op:expr, $inp:expr, $n:expr, $cr:expr, $o:expr) => {
        BenchSpec {
            id: $id,
            tutorial: $tut,
            section: $sec,
            operation: $op,
            input: $inp,
            iters: $n,
            oximedia_crate: Some($cr),
            oxipix_ticket: None,
            rust_op: None,
            oxi_op: Some(Box::new(move || {
                std::hint::black_box($o);
            })),
        }
    };
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

/// Warm-up + median iteration timing in milliseconds.
pub fn time_ms(iters: usize, mut f: impl FnMut()) -> f64 {
    for _ in 0..3 {
        f();
        black_box(());
    }
    let mut samples = Vec::with_capacity(iters.max(1));
    for _ in 0..iters.max(1) {
        let start = Instant::now();
        f();
        black_box(());
        samples.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    samples.sort_by(f64::total_cmp);
    round3(samples[samples.len() / 2])
}

/// Runs a `BenchSpec` once to check parity/execution, times both backends, and joins OpenCV timing.
pub fn evaluate(spec: BenchSpec<'_>, ocv: &BTreeMap<String, f64>) -> BenchRow {
    let opencv_ms = ocv.get(spec.id).copied().map(round3);
    let rust_ms = spec.rust_op.map(|op| time_ms(spec.iters, op));
    let oximedia_ms = spec.oxi_op.map(|op| time_ms(spec.iters, op));
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
    }
}
