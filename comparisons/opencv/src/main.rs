//! Release benchmark runner merging OpenCV 5.1.0, pure Rust, and `oximedia-cv`/`oxipix` timings.

mod cases_basic;
mod cases_imgproc;
mod spec;

#[allow(dead_code)]
#[path = "../../../snippets/examples/core/adding_images.rs"]
pub mod adding_images;
#[allow(dead_code)]
#[path = "../../../snippets/examples/core/basic_linear_transform.rs"]
pub mod basic_linear_transform;
#[allow(dead_code)]
#[path = "../../../snippets/examples/features/brief_match.rs"]
pub mod brief_match;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_edges/canny.rs"]
pub mod canny;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_edges/contours.rs"]
pub mod contours;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_morph/distance.rs"]
pub mod distance;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_color/equalize.rs"]
pub mod equalize;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_morph/erosion.rs"]
pub mod erosion;
#[allow(dead_code)]
#[path = "../../../snippets/examples/features/fast.rs"]
pub mod fast;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_filters/filter2d.rs"]
pub mod filter2d;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_morph/hats.rs"]
pub mod hats;
#[allow(dead_code)]
#[path = "../../../snippets/examples/features/homography.rs"]
pub mod homography;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_edges/hough_lines.rs"]
pub mod hough_lines;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_color/in_range.rs"]
pub mod in_range;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_filters/laplace.rs"]
pub mod laplace;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia_core/adding_images.rs"]
pub mod oxi_adding_images;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia/canny.rs"]
pub mod oxi_canny;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia_imgproc/equalize.rs"]
pub mod oxi_equalize;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia_imgproc/erosion.rs"]
pub mod oxi_erosion;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia_imgproc/filter2d.rs"]
pub mod oxi_filter2d;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia/hough_circles.rs"]
pub mod oxi_hough_circles;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia/hough_lines.rs"]
pub mod oxi_hough_lines;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia_imgproc/in_range.rs"]
pub mod oxi_in_range;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia/laplacian.rs"]
pub mod oxi_laplacian;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia/morphology.rs"]
pub mod oxi_morphology;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia/orb.rs"]
pub mod oxi_orb;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia/orb_match.rs"]
pub mod oxi_orb_match;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia_imgproc/pyramids.rs"]
pub mod oxi_pyramids;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia/quality.rs"]
pub mod oxi_quality;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia/smoothing.rs"]
pub mod oxi_smoothing;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia_imgproc/sobel.rs"]
pub mod oxi_sobel;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oximedia_imgproc/warp_affine.rs"]
pub mod oxi_warp_affine;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oxipix/contours.rs"]
pub mod oxipix_contours;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oxipix/homography.rs"]
pub mod oxipix_homography;
#[allow(dead_code)]
#[path = "../../../snippets/examples/oxipix/threshold.rs"]
pub mod oxipix_threshold;
#[allow(dead_code)]
#[path = "../../../snippets/examples/app/psnr.rs"]
pub mod psnr;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_geom/pyramids.rs"]
pub mod pyramids;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_filters/smoothing.rs"]
pub mod smoothing;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_filters/sobel.rs"]
pub mod sobel;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_geom/template_matching.rs"]
pub mod template_matching;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_color/threshold.rs"]
pub mod threshold;
#[allow(dead_code)]
#[path = "../../../snippets/examples/imgproc_geom/warp_affine.rs"]
pub mod warp_affine;

use serde::Deserialize;
use spec::{Assets, BenchmarkCatalog, Digest, Environment, OpenCvData, calibration, evaluate};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct OpenCvOutput {
    opencv_version: String,
    #[serde(default)]
    build_info: Vec<String>,
    results: BTreeMap<String, f64>,
    #[serde(default)]
    digests: BTreeMap<String, Digest>,
}

/// Timings of an optimised-build run, merged into the default run as `*_opt_ms`.
#[derive(Deserialize)]
struct OptRow {
    id: String,
    rust_ms: Option<f64>,
    oximedia_ms: Option<f64>,
}

#[derive(Deserialize)]
struct OptCatalog {
    benchmarks: Vec<OptRow>,
}

fn load_opt(path: Option<&String>) -> BTreeMap<String, OptRow> {
    let Some(text) = path.and_then(|p| fs::read_to_string(p).ok()) else {
        return BTreeMap::new();
    };
    let parsed: OptCatalog = serde_json::from_str(&text).expect("parse optimised-build JSON");
    parsed.benchmarks.into_iter().map(|r| (r.id.clone(), r)).collect()
}

#[derive(Deserialize)]
struct ExistingCatalog {
    #[serde(default)]
    benchmarks: Vec<ExistingRow>,
}

#[derive(Deserialize)]
struct ExistingRow {
    id: String,
    warning_note: Option<String>,
}

fn load_notes(path: &Path) -> BTreeMap<String, String> {
    let Ok(text) = fs::read_to_string(path) else {
        return BTreeMap::new();
    };
    let Ok(parsed) = serde_json::from_str::<ExistingCatalog>(&text) else {
        return BTreeMap::new();
    };
    parsed
        .benchmarks
        .into_iter()
        .filter_map(|r| r.warning_note.map(|n| (r.id, n)))
        .collect()
}

fn load_opencv(path: &Path) -> OpenCvOutput {
    let text = fs::read_to_string(path).expect("read OpenCV JSON output");
    serde_json::from_str(&text).expect("parse OpenCV JSON output")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ocv_path = Path::new(args.get(1).map_or("/tmp/opencv_bench.json", String::as_str));
    let out_path = Path::new(
        args.get(2)
            .map_or("../../catalog/benchmarks.json", String::as_str),
    );
    let opt = load_opt(args.get(3));
    let ocv = load_opencv(ocv_path);
    let data = OpenCvData {
        results: &ocv.results,
        digests: &ocv.digests,
    };
    let notes = load_notes(out_path);
    let assets = Assets::load();
    let mut benchmarks = Vec::new();
    for spec in cases_basic::collect(&assets)
        .into_iter()
        .chain(cases_imgproc::collect(&assets))
    {
        eprintln!("Running {}", spec.id);
        let mut row = evaluate(spec, &data);
        if let Some(o) = opt.get(&row.id) {
            row.rust_opt_ms = o.rust_ms;
            row.oximedia_opt_ms = o.oximedia_ms;
        }
        row.warning_note = notes.get(&row.id).cloned();
        benchmarks.push(row);
    }
    let catalog = BenchmarkCatalog {
        warn_ratio_threshold: 3.0,
        opencv_warn_ratio_threshold: 20.0,
        environment: Environment {
            os: std::env::consts::OS.into(),
            arch: std::env::consts::ARCH.into(),
            opencv_version: ocv.opencv_version,
            rust_profile: "release".into(),
            oximedia_cv_version: "0.2.1".into(),
            oxipix_version: "0.1.0".into(),
            imageproc_version: "0.27.0".into(),
            builds: if opt.is_empty() {
                vec!["release".into()]
            } else {
                vec!["release".into(), "release+fat-lto+cgu1+native".into()]
            },
            opencv_build: ocv.build_info,
        },
        calibration: calibration(),
        benchmarks,
    };
    let json = serde_json::to_string_pretty(&catalog).expect("serialize benchmarks");
    fs::write(out_path, format!("{json}\n")).expect("write catalog/benchmarks.json");
    eprintln!(
        "Wrote {} benchmark rows to {}",
        catalog.benchmarks.len(),
        out_path.display()
    );
}

#[cfg(test)]
mod tests {
    use crate::spec::{Digest, calibration, parity, ratio, round3, time_ms, time_stats};
    use rstest::rstest;

    #[rstest]
    #[case(Some(10.0), Some(2.0), Some(5.0))]
    #[case(Some(1.0), Some(0.0), None)]
    #[case(None, Some(2.0), None)]
    fn ratios_handle_zero_and_missing(
        #[case] a: Option<f64>,
        #[case] b: Option<f64>,
        #[case] expected: Option<f64>,
    ) {
        assert_eq!(ratio(a, b), expected);
    }

    fn digest(w: u32, h: u32, mean: f64) -> Option<Digest> {
        Some(Digest { w, h, mean })
    }

    #[rstest]
    #[case(digest(10, 10, 100.0), digest(10, 10, 101.5), "ok")]
    #[case(digest(10, 10, 100.0), digest(10, 10, 120.0), "mismatch")]
    #[case(digest(10, 10, 100.0), digest(10, 11, 100.0), "mismatch")]
    #[case(digest(250, 1, 0.0), digest(200, 1, 0.0), "ok")]
    #[case(digest(250, 1, 0.0), digest(100, 1, 0.0), "mismatch")]
    #[case(None, digest(10, 10, 1.0), "n/a")]
    fn parity_compares_size_mean_and_counts(
        #[case] a: Option<Digest>,
        #[case] b: Option<Digest>,
        #[case] expected: &str,
    ) {
        assert_eq!(parity(a, b), expected);
    }

    #[test]
    fn stats_are_ordered_and_adaptive() {
        let stats = time_stats(5, || {
            std::hint::black_box(2u64.pow(3));
        });
        assert!(stats.p10 <= stats.median && stats.median <= stats.p90);
        assert!(stats.iters >= 5 && stats.noise() >= 0.0);
    }

    #[test]
    fn calibration_has_floor_rows() {
        let floor = calibration();
        assert!(floor.contains_key("empty_closure") && floor["alloc_640x480x3"] >= 0.0);
    }

    #[test]
    fn rounding_and_timing_produce_non_negative_values() {
        assert_eq!(round3(1.23456), 1.235);
        assert!(
            time_ms(3, || {
                std::hint::black_box(1u64.wrapping_add(1));
            }) >= 0.0
        );
    }
}
