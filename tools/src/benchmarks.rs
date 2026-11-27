//! Benchmark catalog verification and Markdown rendering for `website/docs/benchmarks.md` and tutorial pages.

use crate::generate::Page;
use crate::model::{Catalog, Coverage, Entry, Status};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

fn default_warn() -> f64 {
    3.0
}

fn default_opencv_warn() -> f64 {
    20.0
}

/// Environment metadata recorded alongside `catalog/benchmarks.json`.
#[derive(Debug, Default, Deserialize)]
pub struct Environment {
    #[serde(default)]
    pub os: String,
    #[serde(default)]
    pub arch: String,
    #[serde(default)]
    pub opencv_version: String,
    #[serde(default)]
    pub rust_profile: String,
    #[serde(default)]
    pub oximedia_cv_version: String,
    #[serde(default)]
    pub oxipix_version: String,
    #[serde(default)]
    pub imageproc_version: String,
    /// Where the `oxipix` imgproc backend comes from.
    #[serde(default)]
    pub oxipix_next_source: String,
    /// Threads OpenCV uses by default on the benchmark machine.
    #[serde(default)]
    pub opencv_threads: usize,
    /// Whether OpenCV found an OpenCL device.
    #[serde(default)]
    pub opencv_opencl: bool,
}

/// Spread of one timing series (milliseconds), as written by the benchmark harness.
#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct Stats {
    #[serde(default)]
    pub median: f64,
    #[serde(default)]
    pub mad: f64,
}

impl Stats {
    /// Relative noise floor `2 * MAD / median`: a gap smaller than this is not a real difference.
    pub fn noise(&self) -> f64 {
        if self.median > 0.0 {
            2.0 * self.mad / self.median
        } else {
            0.0
        }
    }
}

/// One measured comparison row across OpenCV, pure Rust, and `oximedia-cv` / `oxipix`.
#[derive(Debug, Default, Deserialize)]
pub struct BenchRow {
    pub id: String,
    pub tutorial: String,
    pub section: String,
    pub operation: String,
    pub input: String,
    #[serde(default)]
    pub opencv_ms: Option<f64>,
    #[serde(default)]
    pub rust_ms: Option<f64>,
    #[serde(default)]
    pub oximedia_ms: Option<f64>,
    #[serde(default)]
    pub oximedia_crate: Option<String>,
    #[serde(default)]
    pub oximedia_vs_rust: Option<f64>,
    #[serde(default)]
    pub rust_vs_opencv: Option<f64>,
    #[serde(default)]
    pub oximedia_vs_opencv: Option<f64>,
    #[serde(default)]
    pub oxipix_ticket: Option<String>,
    #[serde(default)]
    pub warning_note: Option<String>,
    /// OpenCV timed with a freshly allocated output, comparable with the Rust snippets.
    #[serde(default)]
    pub opencv_fresh_ms: Option<f64>,
    /// OpenCV with its default number of threads.
    #[serde(default)]
    pub opencv_mt_ms: Option<f64>,
    #[serde(default)]
    pub rust_opt_ms: Option<f64>,
    #[serde(default)]
    pub oximedia_opt_ms: Option<f64>,
    #[serde(default)]
    pub rust_stats: Option<Stats>,
    #[serde(default)]
    pub oximedia_stats: Option<Stats>,
    /// The `oxipix` imgproc kernels (git branch) measured in the same run.
    #[serde(default)]
    pub oxipix_next_ms: Option<f64>,
    #[serde(default)]
    pub oxipix_next_opt_ms: Option<f64>,
    /// The same kernels on all threads (`parallel` feature).
    #[serde(default)]
    pub oxipix_next_mt_ms: Option<f64>,
    /// The guide's `opencv` crate example (with the `image` <-> `Mat` copies) on one thread and on all threads.
    #[serde(default)]
    pub opencv_crate_ms: Option<f64>,
    #[serde(default)]
    pub opencv_crate_mt_ms: Option<f64>,
    #[serde(default)]
    pub oxipix_next_stats: Option<Stats>,
    /// `ok`, `mismatch` (backends do different work) or `n/a`.
    #[serde(default)]
    pub parity: String,
}

/// Deserialized `catalog/benchmarks.json`.
#[derive(Debug, Deserialize)]
pub struct Benchmarks {
    #[serde(default = "default_warn")]
    pub warn_ratio_threshold: f64,
    #[serde(default = "default_opencv_warn")]
    pub opencv_warn_ratio_threshold: f64,
    #[serde(default)]
    pub environment: Environment,
    #[serde(default)]
    pub benchmarks: Vec<BenchRow>,
    /// Small, medium and big pictures for the kernels where size and threads matter.
    #[serde(default)]
    pub size_sweep: Vec<BenchRow>,
}

impl Default for Benchmarks {
    fn default() -> Self {
        Self {
            warn_ratio_threshold: default_warn(),
            opencv_warn_ratio_threshold: default_opencv_warn(),
            environment: Environment::default(),
            benchmarks: Vec::new(),
            size_sweep: Vec::new(),
        }
    }
}

/// Returns true if `ticket` has the canonical `OXI-<digits>` shape.
pub fn valid_ticket(ticket: &str) -> bool {
    let Some(num) = ticket.strip_prefix("OXI-") else {
        return false;
    };
    !num.is_empty() && num.chars().all(|c| c.is_ascii_digit())
}

/// True when `row` exhibits a significant performance divergence between backends.
pub fn row_diverges(row: &BenchRow, bench: &Benchmarks) -> bool {
    let vs_rust = row
        .oximedia_vs_rust
        .is_some_and(|r| r >= bench.warn_ratio_threshold);
    let rust_slower = match (row.rust_ms, row.oximedia_ms) {
        (Some(r), Some(o)) if o > 0.0 => r / o >= bench.warn_ratio_threshold,
        _ => false,
    };
    let vs_ocv = row
        .oximedia_vs_opencv
        .is_some_and(|r| r >= bench.opencv_warn_ratio_threshold)
        || row
            .rust_vs_opencv
            .is_some_and(|r| r >= bench.opencv_warn_ratio_threshold);
    vs_rust || rust_slower || vs_ocv
}

fn has_oximedia_or_oxipix(entry: &Entry) -> bool {
    entry
        .crates
        .iter()
        .any(|c| c == "oximedia-cv" || c == "oxipix")
}

fn row_problems<'a>(
    bench: &'a Benchmarks,
    known: &BTreeSet<&str>,
    by_tutorial: &mut BTreeSet<&'a str>,
) -> Vec<String> {
    let mut out = Vec::new();
    for row in &bench.benchmarks {
        by_tutorial.insert(row.tutorial.as_str());
        if !known.contains(row.tutorial.as_str()) {
            out.push(format!(
                "benchmark row {} references unknown tutorial {}",
                row.id, row.tutorial
            ));
        }
        let missing_note = row
            .warning_note
            .as_deref()
            .is_none_or(|n| n.trim().is_empty());
        if row_diverges(row, bench) && missing_note {
            out.push(format!(
                "benchmark row {} exceeds divergence threshold without a warning_note",
                row.id
            ));
        }
    }
    out
}

fn coverage_problems(coverage: &Coverage, by_tutorial: &BTreeSet<&str>) -> Vec<String> {
    let mut out = Vec::new();
    for (id, entry) in &coverage.tutorials {
        if let Some(ticket) = entry.oxipix_ticket.as_deref()
            && !valid_ticket(ticket)
        {
            out.push(format!("tutorial {id} has invalid oxipix_ticket {ticket}"));
        }
        if !matches!(entry.status, Status::Covered | Status::Partial) {
            continue;
        }
        if !by_tutorial.contains(id.as_str()) {
            out.push(format!(
                "tutorial {id} is {} but has no entry in catalog/benchmarks.json",
                entry.status.label()
            ));
        }
        let has_ticket = entry.oxipix_ticket.as_deref().is_some_and(valid_ticket);
        if !has_oximedia_or_oxipix(entry) && !has_ticket {
            out.push(format!(
                "tutorial {id} lacks oximedia-cv/oxipix and has no oxipix_ticket"
            ));
        }
    }
    out
}

/// Checks parity between `catalog/coverage/*.yaml` and `catalog/benchmarks.json`.
pub fn problems(catalog: &Catalog, coverage: &Coverage, bench: &Benchmarks) -> Vec<String> {
    let known: BTreeSet<&str> = catalog.tutorials.iter().map(|t| t.id.as_str()).collect();
    let mut by_tutorial = BTreeSet::new();
    let mut out = row_problems(bench, &known, &mut by_tutorial);
    out.extend(coverage_problems(coverage, &by_tutorial));
    out
}

fn fmt_ms(ms: Option<f64>) -> String {
    ms.map_or_else(|| "—".into(), |v| format!("{v:.3} ms"))
}

fn fmt_ratio(r: Option<f64>) -> String {
    r.map_or_else(|| "—".into(), |v| format!("{v:.2}x"))
}

fn oxi_cell(r: &BenchRow) -> String {
    match (r.oximedia_ms, r.oxipix_ticket.as_deref()) {
        (Some(_), _) => fmt_ms(r.oximedia_ms),
        (None, Some(t)) => format!("— ({t})"),
        (None, None) => "—".into(),
    }
}

/// Makes `text` safe inside a Markdown/MDX table cell: `|` would end the cell and a bare `<` would start a JSX tag.
pub(crate) fn clean(text: &str) -> String {
    text.replace('|', "\\|").replace('<', "&lt;")
}

/// The environment line for the `oxipix` imgproc backend, or nothing when the catalog has none.
fn next_line(source: &str) -> String {
    if source.is_empty() {
        String::new()
    } else {
        format!("- **oxipix imgproc backend:** {source}\n")
    }
}

/// Renders the tutorial's per-page performance table and automatic divergence warning, if any.
pub fn tutorial_section(tutorial_id: &str, bench: &Benchmarks) -> String {
    let rows: Vec<&BenchRow> = bench
        .benchmarks
        .iter()
        .filter(|r| r.tutorial == tutorial_id)
        .collect();
    if rows.is_empty() {
        return String::new();
    }
    let mut lines = vec![
        "\n## Performance comparison\n".to_string(),
        "| Operation | Input | OpenCV | Pure Rust | oximedia / oxipix | oxi / Rust |".to_string(),
        "|---|---|---|---|---|---|".to_string(),
    ];
    let mut notes = Vec::new();
    for r in rows {
        lines.push(format!(
            "| {} | {} | {} | {} | {} | {} |",
            clean(&r.operation),
            clean(&r.input),
            fmt_ms(r.opencv_ms),
            fmt_ms(r.rust_ms),
            oxi_cell(r),
            fmt_ratio(r.oximedia_vs_rust)
        ));
        if let Some(note) = r.warning_note.as_deref().filter(|n| !n.trim().is_empty()) {
            notes.push(format!("- **{}:** {note}", r.operation));
        }
    }
    if !notes.is_empty() {
        lines.push(String::new());
        lines.push(":::warning Performance difference".into());
        lines.push(String::new());
        lines.extend(notes);
        lines.push(String::new());
        lines.push(":::".into());
    }
    lines.push(String::new());
    lines.join("\n")
}

/// Renders the site-wide `website/docs/benchmarks.md` reference page.
pub fn page(catalog: &Catalog, bench: &Benchmarks) -> Page {
    let titles: BTreeMap<&str, &str> = catalog
        .tutorials
        .iter()
        .map(|t| (t.id.as_str(), t.title.as_str()))
        .collect();
    let e = &bench.environment;
    let mut lines = vec![
        "---\ntitle: Performance benchmarks\n---\n".into(),
        "Incremental performance reference comparing native **OpenCV**, pure Rust (`image` + `imageproc`), and **`oximedia-cv` / `oxipix`** across every implemented tutorial.\n".into(),
        format!(
            "- **Host / profile:** {} {} (`cargo run --{}`)\n- **OpenCV:** {}\n- **Crates:** `oximedia-cv` {}, `oxipix` {}, `imageproc` {}\n{}- **Divergence thresholds:** >= {:.1}x (`oximedia` vs pure Rust) or >= {:.1}x vs OpenCV triggers an automated warning callout.\n",
            e.os, e.arch, e.rust_profile, e.opencv_version, e.oximedia_cv_version, e.oxipix_version, e.imageproc_version, next_line(&e.oxipix_next_source), bench.warn_ratio_threshold, bench.opencv_warn_ratio_threshold
        ),
        "| Tutorial | Operation | Input | OpenCV | Pure Rust | oximedia / oxipix | oxi / Rust | Notes |".into(),
        "|---|---|---|---|---|---|---|---|".into(),
    ];
    for r in &bench.benchmarks {
        let title = titles
            .get(r.tutorial.as_str())
            .copied()
            .unwrap_or(&r.tutorial);
        let link = format!(
            "[{}](opencv/{}/{}.mdx)",
            clean(title),
            r.section,
            r.tutorial
        );
        let note = r.warning_note.as_deref().unwrap_or("—");
        lines.push(format!(
            "| {link} | {} | {} | {} | {} | {} | {} | {} |",
            clean(&r.operation),
            clean(&r.input),
            fmt_ms(r.opencv_ms),
            fmt_ms(r.rust_ms),
            oxi_cell(r),
            fmt_ratio(r.oximedia_vs_rust),
            clean(note)
        ));
    }
    let summary = crate::perf_queue::summary(&crate::perf_queue::build(bench, &Default::default()));
    Page {
        path: "docs/benchmarks.md".into(),
        content: format!("{}\n\n{summary}", lines.join("\n")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::fixtures::{catalog, coverage};
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    #[case("git branch x", "- **oxipix imgproc backend:** git branch x\n")]
    fn the_oxipix_imgproc_environment_line_is_optional(#[case] source: &str, #[case] want: &str) {
        assert_eq!(next_line(source), want);
    }

    #[rstest]
    #[case("a | b", "a \\| b")]
    #[case("<= 32 bits", "&lt;= 32 bits")]
    #[case("plain", "plain")]
    fn table_cells_are_escaped(#[case] text: &str, #[case] want: &str) {
        assert_eq!(clean(text), want);
    }

    #[rstest]
    #[case(0.0, 1.0, 0.0)]
    #[case(1.0, 0.1, 0.2)]
    fn noise_is_twice_the_relative_mad(#[case] median: f64, #[case] mad: f64, #[case] want: f64) {
        assert!((Stats { median, mad }.noise() - want).abs() < 1e-9);
    }

    fn sample_row(
        id: &str,
        tut: &str,
        r: Option<f64>,
        o: Option<f64>,
        note: Option<&str>,
    ) -> BenchRow {
        let ratio = match (r, o) {
            (Some(rv), Some(ov)) if rv > 0.0 => Some(ov / rv),
            _ => None,
        };
        BenchRow {
            id: id.into(),
            tutorial: tut.into(),
            section: "core".into(),
            operation: "op | 1".into(),
            input: "4x4".into(),
            opencv_ms: Some(0.1),
            rust_ms: r,
            oximedia_ms: o,
            oximedia_crate: Some("oximedia-cv".into()),
            oximedia_vs_rust: ratio,
            rust_vs_opencv: r.map(|v| v / 0.1),
            oximedia_vs_opencv: o.map(|v| v / 0.1),
            oxipix_ticket: None,
            warning_note: note.map(str::to_string),
            ..BenchRow::default()
        }
    }

    #[rstest]
    #[case("OXI-87", true)]
    #[case("OXI-1", true)]
    #[case("OXI-", false)]
    #[case("OXI-abc", false)]
    #[case("DEN-1", false)]
    fn ticket_validation(#[case] s: &str, #[case] ok: bool) {
        assert_eq!(valid_ticket(s), ok);
    }

    #[test]
    fn problems_detect_missing_and_divergent_rows() {
        let bench = Benchmarks::default();
        let mut cov = coverage(Status::Covered, &["ndarray"]);
        cov.tutorials.get_mut("tutorial_mat").unwrap().oxipix_ticket = Some("BAD".into());
        let found = problems(&catalog(), &cov, &bench);
        assert!(found.iter().any(|p| p.contains("invalid oxipix_ticket")));
        assert!(
            found
                .iter()
                .any(|p| p.contains("no entry in catalog/benchmarks.json"))
        );
        assert!(found.iter().any(|p| p.contains("no oxipix_ticket")));

        let div_bench = Benchmarks {
            benchmarks: vec![sample_row("x", "unknown", Some(1.0), Some(5.0), None)],
            ..Benchmarks::default()
        };
        let found2 = problems(&catalog(), &cov, &div_bench);
        assert!(
            found2
                .iter()
                .any(|p| p.contains("unknown tutorial unknown"))
        );
        assert!(found2.iter().any(|p| p.contains("without a warning_note")));
    }

    #[test]
    fn rust_slower_also_triggers_divergence() {
        let bench = Benchmarks::default();
        let r = sample_row("sobel", "tutorial_mat", Some(4.0), Some(1.0), None);
        assert!(row_diverges(&r, &bench));
        assert!(!row_diverges(
            &sample_row("ok", "tutorial_mat", Some(1.0), Some(0.0), None),
            &bench
        ));
    }

    #[test]
    fn valid_oximedia_entry_has_no_problems() {
        let cov = coverage(Status::Covered, &["oximedia-cv"]);
        let row = BenchRow {
            oximedia_vs_opencv: Some(25.0),
            ..sample_row(
                "ok",
                "tutorial_mat",
                Some(1.0),
                Some(1.0),
                Some("slow vs ocv"),
            )
        };
        let bench = Benchmarks {
            benchmarks: vec![row],
            ..Benchmarks::default()
        };
        assert!(problems(&catalog(), &cov, &bench).is_empty());
    }

    #[test]
    fn renders_page_and_tutorial_section_with_warning() {
        let row = sample_row("s", "tutorial_mat", Some(1.0), Some(5.0), Some("too slow"));
        let plain = BenchRow {
            oximedia_ms: None,
            rust_ms: None,
            opencv_ms: None,
            oxipix_ticket: None,
            warning_note: None,
            oximedia_vs_rust: None,
            rust_vs_opencv: None,
            oximedia_vs_opencv: None,
            ..sample_row("p", "plain", None, None, None)
        };
        let missing = BenchRow {
            oximedia_ms: None,
            oxipix_ticket: Some("OXI-88".into()),
            ..sample_row("m", "other", Some(1.0), None, None)
        };
        let bench = Benchmarks {
            benchmarks: vec![row, plain, missing],
            ..Benchmarks::default()
        };
        assert_eq!(tutorial_section("absent", &bench), "");
        assert!(!tutorial_section("plain", &bench).contains(":::warning"));
        let sec = tutorial_section("tutorial_mat", &bench);
        assert!(
            sec.contains("## Performance comparison")
                && sec.contains(":::warning Performance difference")
                && sec.contains("too slow")
        );
        let p = page(&catalog(), &bench);
        assert!(
            p.path == "docs/benchmarks.md"
                && p.content.contains("— (OXI-88)")
                && p.content.contains("| — | — | — |")
                && p.content.contains("op \\| 1")
        );
    }
}
