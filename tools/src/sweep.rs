//! The "Threads and picture size" page: small, medium and big pictures, with OpenCV on one thread and on all
//! the threads it uses by default, against the best Rust backend.

use crate::benchmarks::{BenchRow, Benchmarks, clean};
use crate::generate::Page;

/// The size class of a sweep row id such as `sweep_gaussian_big`.
fn size_of(id: &str) -> &str {
    id.rsplit('_').next().unwrap_or("")
}

/// The kernel of a sweep row id such as `sweep_gaussian_big` (`gaussian`).
fn kernel_of(id: &str) -> &str {
    let trimmed = id.strip_prefix("sweep_").unwrap_or(id);
    trimmed
        .rsplit_once('_')
        .map_or(trimmed, |(kernel, _)| kernel)
}

/// Position of a size class in reading order (unknown sizes last).
fn size_rank(size: &str) -> usize {
    ["small", "medium", "big"]
        .iter()
        .position(|s| *s == size)
        .unwrap_or(3)
}

fn ms(value: Option<f64>) -> String {
    value.map_or("—".into(), |v| format!("{v:.2} ms"))
}

/// `value / reference` as `1.3x`, or a dash when either is missing.
fn gap(value: Option<f64>, reference: Option<f64>) -> String {
    match (value, reference) {
        (Some(v), Some(r)) if r > 0.0 => format!("{:.1}x", v / r),
        _ => "—".into(),
    }
}

/// The faster of the two Rust backends.
fn best_rust(row: &BenchRow) -> Option<f64> {
    [row.rust_ms, row.oxipix_next_ms]
        .into_iter()
        .flatten()
        .reduce(f64::min)
}

fn table_row(row: &BenchRow) -> String {
    let one = row.opencv_fresh_ms.or(row.opencv_ms);
    let best = best_rust(row);
    format!(
        "| {} | {} | {} | {} ({}) | {} | {} | {} | {} |",
        clean(&row.operation),
        clean(&row.input),
        ms(one),
        ms(row.opencv_mt_ms),
        gap(one, row.opencv_mt_ms).replace('x', "x faster"),
        ms(row.rust_ms),
        ms(row.oxipix_next_ms),
        gap(best, one),
        gap(best, row.opencv_mt_ms),
    )
}

const INTRO: &str = "OpenCV runs on all the threads it finds by default, while the algorithm tables on the other benchmark pages pin it to one thread so that algorithms are compared, not core counts. This page puts the two side by side on small (640x480), medium (1920x1080) and big (3840x2160) pictures. The Rust columns are single-threaded; the last two columns say how far the best of them is from OpenCV on one thread and on all threads.\n";

/// The page, or `None` when the catalog has no size sweep.
#[must_use]
pub fn page(bench: &Benchmarks) -> Option<Page> {
    if bench.size_sweep.is_empty() {
        return None;
    }
    let mut rows: Vec<&BenchRow> = bench.size_sweep.iter().collect();
    let order: Vec<&str> = rows
        .iter()
        .map(|r| kernel_of(&r.id))
        .fold(Vec::new(), |mut seen, k| {
            if !seen.contains(&k) {
                seen.push(k);
            }
            seen
        });
    rows.sort_by_key(|r| {
        (
            order.iter().position(|k| *k == kernel_of(&r.id)),
            size_rank(size_of(&r.id)),
        )
    });
    let e = &bench.environment;
    let mut lines = vec![
        "---\ntitle: Threads and picture size\n---\n".to_string(),
        INTRO.to_string(),
        format!(
            "- **OpenCV threads by default:** {}\n- **OpenCV found an OpenCL device:** {}\n",
            e.opencv_threads,
            if e.opencv_opencl { "yes" } else { "no" }
        ),
        "| Operation | Picture | OpenCV, 1 thread | OpenCV, all threads | imageproc | oxipix | Best Rust vs OpenCV 1 thread | Best Rust vs OpenCV all threads |".into(),
        "|---|---|---|---|---|---|---|---|".into(),
    ];
    lines.extend(rows.into_iter().map(table_row));
    Some(Page {
        path: "docs/threads-and-size.md".into(),
        content: format!("{}\n", lines.join("\n")),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn row(id: &str, one: f64, mt: f64, rust: Option<f64>, next: Option<f64>) -> BenchRow {
        BenchRow {
            id: id.into(),
            operation: "Blur | x".into(),
            input: "big (3840x2160)".into(),
            opencv_ms: Some(one),
            opencv_mt_ms: Some(mt),
            rust_ms: rust,
            oxipix_next_ms: next,
            ..BenchRow::default()
        }
    }

    #[rstest]
    #[case("sweep_gaussian_big", "gaussian", "big")]
    #[case("sweep_warp_affine_small", "warp_affine", "small")]
    #[case("plain", "plain", "plain")]
    fn ids_split_into_kernel_and_size(#[case] id: &str, #[case] kernel: &str, #[case] size: &str) {
        assert_eq!((kernel_of(id), size_of(id)), (kernel, size));
    }

    #[rstest]
    #[case("small", 0)]
    #[case("big", 2)]
    #[case("huge", 3)]
    fn sizes_have_a_reading_order(#[case] size: &str, #[case] rank: usize) {
        assert_eq!(size_rank(size), rank);
    }

    #[rstest]
    #[case(Some(4.0), Some(2.0), "2.0x")]
    #[case(Some(4.0), Some(0.0), "—")]
    #[case(None, Some(2.0), "—")]
    fn gaps_are_ratios(
        #[case] value: Option<f64>,
        #[case] reference: Option<f64>,
        #[case] want: &str,
    ) {
        assert_eq!(gap(value, reference), want);
    }

    #[test]
    fn the_page_groups_kernels_and_orders_sizes() {
        let bench = Benchmarks {
            size_sweep: vec![
                row("sweep_blur_big", 8.0, 2.0, Some(40.0), Some(10.0)),
                row("sweep_blur_small", 1.0, 0.5, None, Some(1.5)),
                row("sweep_sobel_small", 1.0, 1.0, Some(2.0), None),
            ],
            ..Benchmarks::default()
        };
        let rendered = page(&bench).unwrap();
        assert_eq!(rendered.path, "docs/threads-and-size.md");
        let at = |cell: &str| rendered.content.find(cell).unwrap();
        // Kernels stay together (blur before sobel) and sizes read small before big.
        assert!(at("1.50 ms") < at("10.00 ms") && at("10.00 ms") < at("2.00 ms | —"));
        assert!(
            rendered.content.contains("4.0x faster") && rendered.content.contains("Blur \\| x")
        );
        assert!(page(&Benchmarks::default()).is_none());
    }
}
