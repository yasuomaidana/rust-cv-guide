//! The "Rust only, OpenCV, or both?" page: for every benchmarked operation and picture size, the fastest Rust
//! result, what the `opencv` crate gives for the same call, and a recommendation derived from the gap.

use crate::benchmarks::{BenchRow, Benchmarks, clean};
use crate::generate::Page;

/// Below this gap the Rust version is close enough that the native dependency is not worth it.
const CLOSE: f64 = 1.5;
/// Above this gap the OpenCV call is clearly the better choice for that step.
const CLEARLY_BETTER: f64 = 4.0;
/// A difference smaller than this many milliseconds does not matter whatever the ratio is.
const NEGLIGIBLE_MS: f64 = 0.2;

/// What to do for one operation at one size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Advice {
    /// Rust is as fast or close enough.
    RustOnly,
    /// OpenCV is faster, but only worth the dependency where this step is the bottleneck.
    MixedIfHot,
    /// OpenCV is clearly faster.
    UseOpenCv,
}

impl Advice {
    fn text(self) -> &'static str {
        match self {
            Advice::RustOnly => "Rust only",
            Advice::MixedIfHot => "Rust first; call OpenCV for this step if it is your bottleneck",
            Advice::UseOpenCv => "Mixed approach: call OpenCV for this step",
        }
    }
}

/// The recommendation for a Rust time against the OpenCV time, both in milliseconds.
#[must_use]
pub fn advise(rust_ms: f64, opencv_ms: f64) -> Advice {
    if rust_ms - opencv_ms < NEGLIGIBLE_MS || rust_ms <= opencv_ms * CLOSE {
        Advice::RustOnly
    } else if rust_ms <= opencv_ms * CLEARLY_BETTER {
        Advice::MixedIfHot
    } else {
        Advice::UseOpenCv
    }
}

/// The fastest Rust result of a row and which backend it came from.
fn best_rust(row: &BenchRow) -> Option<(f64, &'static str)> {
    [
        (row.rust_ms, "imageproc"),
        (row.oxipix_next_ms, "oxipix"),
        (row.oxipix_next_mt_ms, "oxipix, threads"),
    ]
    .into_iter()
    .filter_map(|(ms, name)| ms.map(|v| (v, name)))
    .min_by(|a, b| a.0.total_cmp(&b.0))
}

/// What a Rust user gets from the `opencv` crate: all threads, which is OpenCV's default.
fn crate_ms(row: &BenchRow) -> Option<f64> {
    row.opencv_crate_mt_ms.or(row.opencv_crate_ms)
}

fn table_row(row: &BenchRow) -> Option<String> {
    let (rust, backend) = best_rust(row)?;
    let opencv = crate_ms(row)?;
    Some(format!(
        "| {} | {} | {:.2} ms ({backend}) | {:.2} ms | {:.1}x | {} |",
        clean(&row.operation),
        clean(&row.input),
        rust,
        opencv,
        rust / opencv,
        advise(rust, opencv).text()
    ))
}

const INTRO: &str = "The guide shows each tutorial three ways: Rust only (`image` + `imageproc`, or `oxipix` / `oximedia-cv`) and the `opencv` crate, which calls a native OpenCV. This page says, from the benchmarks, where staying in Rust costs little and where OpenCV is worth the dependency. The `opencv` column is what a Rust program gets from the guide's `opencv` example on all threads, including the copies between `image` buffers and OpenCV matrices. The Rust column is the fastest Rust backend measured, including `oxipix` with its `parallel` feature.\n\nThe rule: within 1.5x of OpenCV, or less than 0.2 ms apart, stay in Rust; up to 4x, stay in Rust unless this step is your bottleneck; beyond that, call OpenCV for it.\n";

const TRADE_OFFS: &str = "## What each option costs\n\n| | Rust only | `oxipix` imgproc | `opencv` crate | Mixed |\n|---|---|---|---|---|\n| Native dependency | none | none | OpenCV 4.x or 5.x and clang at build time | OpenCV for the steps that use it |\n| WebAssembly | yes | yes | no | not for the OpenCV steps |\n| Threads | `imageproc` single-threaded | `parallel` feature, automatic size threshold | OpenCV's thread pool | both |\n| Build time and binary size | small | small | large | large |\n| Licence | MIT / Apache-2.0 | MIT / Apache-2.0 | crate MIT, OpenCV Apache-2.0 | both |\n\n## Mixing the two\n\nKeep the pipeline in Rust and call OpenCV only for the steps in the table that say so. Moving a picture between the worlds costs one copy each way (the guide's helpers in `snippets-opencv/src/lib.rs`; measured at under 30% of the call on the guide's photographs, and negligible next to a heavy step). In a hot loop, wrap the `image` buffer without copying with `Mat::new_rows_cols_with_data` for as long as the buffer lives.\n";

/// The page, or `None` when no row has both a Rust and an `opencv` crate measurement.
#[must_use]
pub fn page(bench: &Benchmarks) -> Option<Page> {
    let rows: Vec<String> = bench
        .benchmarks
        .iter()
        .chain(&bench.size_sweep)
        .filter_map(table_row)
        .collect();
    if rows.is_empty() {
        return None;
    }
    let mut lines = vec![
        "---\ntitle: Rust only, OpenCV, or both?\n---\n".to_string(),
        INTRO.to_string(),
        "| Operation | Picture | Fastest Rust | `opencv` crate | Rust vs OpenCV | Recommendation |"
            .into(),
        "|---|---|---|---|---|---|".into(),
    ];
    lines.extend(rows);
    lines.push(format!("\n{TRADE_OFFS}"));
    Some(Page {
        path: "docs/choosing.md".into(),
        content: format!("{}\n", lines.join("\n")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1.0, 1.0, Advice::RustOnly)]
    #[case(0.5, 1.0, Advice::RustOnly)]
    #[case(1.4, 1.0, Advice::RustOnly)]
    #[case(0.25, 0.1, Advice::RustOnly)]
    #[case(0.9, 0.1, Advice::UseOpenCv)]
    #[case(3.0, 1.0, Advice::MixedIfHot)]
    #[case(4.0, 1.0, Advice::MixedIfHot)]
    #[case(40.0, 1.0, Advice::UseOpenCv)]
    fn the_gap_decides_the_advice(#[case] rust: f64, #[case] opencv: f64, #[case] want: Advice) {
        assert_eq!(advise(rust, opencv), want);
    }

    fn row(
        rust: Option<f64>,
        next_mt: Option<f64>,
        ocv: Option<f64>,
        ocv_mt: Option<f64>,
    ) -> BenchRow {
        BenchRow {
            operation: "Blur | x".into(),
            input: "big (3840x2160)".into(),
            rust_ms: rust,
            oxipix_next_mt_ms: next_mt,
            opencv_crate_ms: ocv,
            opencv_crate_mt_ms: ocv_mt,
            ..BenchRow::default()
        }
    }

    #[test]
    fn the_best_rust_backend_and_the_threaded_opencv_time_are_used() {
        let r = row(Some(30.0), Some(8.0), Some(20.0), Some(5.0));
        assert_eq!(best_rust(&r), Some((8.0, "oxipix, threads")));
        assert_eq!(crate_ms(&r), Some(5.0));
        assert_eq!(crate_ms(&row(None, None, Some(7.0), None)), Some(7.0));
        let line = table_row(&r).unwrap();
        assert!(
            line.contains("8.00 ms (oxipix, threads) | 5.00 ms | 1.6x | Rust first"),
            "{line}"
        );
        assert!(line.contains("Blur \\| x"));
        let single = row(Some(3.0), None, None, Some(1.0));
        assert_eq!(best_rust(&single), Some((3.0, "imageproc")));
        let one_thread = BenchRow {
            oxipix_next_ms: Some(2.0),
            ..row(None, None, None, None)
        };
        assert_eq!(best_rust(&one_thread), Some((2.0, "oxipix")));
        assert!(table_row(&row(None, None, None, Some(1.0))).is_none());
        assert!(table_row(&row(Some(1.0), None, None, None)).is_none());
    }

    #[test]
    fn the_page_lists_measured_rows_and_the_trade_offs() {
        let bench = Benchmarks {
            benchmarks: vec![row(Some(9.0), None, None, Some(1.0))],
            size_sweep: vec![
                row(Some(1.0), None, Some(1.0), None),
                row(None, None, None, None),
            ],
            ..Benchmarks::default()
        };
        let page = page(&bench).unwrap();
        assert_eq!(page.path, "docs/choosing.md");
        assert!(
            page.content
                .contains("Mixed approach: call OpenCV for this step")
        );
        assert!(
            page.content.contains("Rust only |")
                && page.content.contains("## What each option costs")
        );
        assert_eq!(page.content.matches("| Blur").count(), 2);
        assert!(super::page(&Benchmarks::default()).is_none());
    }
}
