//! Public performance priority queue built from `catalog/benchmarks.json` and `catalog/perf_queue.yaml`.
//!
//! The queue ranks every operation where the best Rust backend is still slower than OpenCV, links each
//! item to its tracking GitHub issue, and is rendered to `PERF_QUEUE.md` and the website.

use crate::benchmarks::{BenchRow, Benchmarks, Stats, valid_ticket};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A gap smaller than this (and than the measured noise) is not a real difference.
const MIN_GAP: f64 = 0.05;
const ISSUE_PREFIX: &str = "https://github.com/";

/// Hand-kept facts about one queue item, from `catalog/perf_queue.yaml`.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Meta {
    pub status: Option<String>,
    pub root_cause: Option<String>,
    pub fix_target: Option<String>,
    /// Plane ticket (`RUSTCVGUID-n` or `OXI-n`) used by the maintainers.
    pub plane: Option<String>,
    /// Public GitHub issue URL the community works from.
    pub issue: Option<String>,
    /// GitHub user who claimed the item.
    pub owner: Option<String>,
    /// Why the backends legitimately do different work (for example an upstream defect). Without it, a
    /// `mismatch` parity row is a project error that has to be fixed.
    pub known_difference: Option<String>,
}

/// Deserialized `catalog/perf_queue.yaml`.
#[derive(Debug, Default, Deserialize)]
pub struct QueueMeta {
    #[serde(default)]
    pub items: BTreeMap<String, Meta>,
}

/// One crate's timing for an operation and its gap to OpenCV.
#[derive(Debug, Serialize, PartialEq)]
pub struct Backend {
    pub name: String,
    pub ms: f64,
    pub ms_opt: Option<f64>,
    pub gap: f64,
    pub gap_opt: Option<f64>,
    pub noise: f64,
}

impl Backend {
    fn best_ms(&self) -> f64 {
        self.ms_opt.map_or(self.ms, |o| o.min(self.ms))
    }
}

/// One operation in the queue.
#[derive(Debug, Serialize)]
pub struct Item {
    pub rank: usize,
    pub id: String,
    pub tutorial: String,
    pub operation: String,
    pub opencv_ms: f64,
    pub best_crate: String,
    pub best_gap: f64,
    pub ms_lost: f64,
    pub parity: String,
    pub backends: Vec<Backend>,
    pub status: String,
    pub root_cause: Option<String>,
    pub fix_target: Option<String>,
    pub plane: Option<String>,
    pub issue: Option<String>,
    pub owner: Option<String>,
    pub known_difference: Option<String>,
}

/// The ranked queue and the operations already at parity with OpenCV.
#[derive(Debug, Serialize)]
pub struct Queue {
    pub queue: Vec<Item>,
    pub at_parity: Vec<Item>,
}

fn round(v: f64, places: i32) -> f64 {
    let k = 10f64.powi(places);
    (v * k).round() / k
}

fn backend(
    name: &str,
    ms: Option<f64>,
    opt: Option<f64>,
    stats: Option<Stats>,
    fair: f64,
) -> Option<Backend> {
    let ms = ms?;
    Some(Backend {
        name: name.into(),
        ms,
        ms_opt: opt,
        gap: round(ms / fair, 2),
        gap_opt: opt.map(|o| round(o / fair, 2)),
        noise: stats.map_or(0.0, |s| s.noise()),
    })
}

fn backends(row: &BenchRow, fair: f64) -> Vec<Backend> {
    let oxi = row.oximedia_crate.as_deref().unwrap_or("oximedia-cv");
    [
        backend(
            "image/imageproc",
            row.rust_ms,
            row.rust_opt_ms,
            row.rust_stats,
            fair,
        ),
        backend(
            oxi,
            row.oximedia_ms,
            row.oximedia_opt_ms,
            row.oximedia_stats,
            fair,
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn default_status(row: &BenchRow, meta: &Meta) -> &'static str {
    match (row.parity == "mismatch", meta.known_difference.is_some()) {
        (true, true) => "known-difference",
        (true, false) => "probe-mismatch",
        _ => "unprobed",
    }
}

fn item(row: &BenchRow, meta: &Meta) -> Option<Item> {
    let fair = row.opencv_fresh_ms.or(row.opencv_ms).filter(|f| *f > 0.0)?;
    let backends = backends(row, fair);
    let best = backends
        .iter()
        .min_by(|a, b| a.best_ms().total_cmp(&b.best_ms()))?;
    let (best_crate, best_gap) = (best.name.clone(), round(best.best_ms() / fair, 2));
    let ms_lost = round(best.best_ms() - fair, 3);
    Some(Item {
        rank: 0,
        id: row.id.clone(),
        tutorial: row.tutorial.clone(),
        operation: row.operation.clone(),
        opencv_ms: fair,
        best_crate,
        best_gap,
        ms_lost,
        parity: row.parity.clone(),
        backends,
        status: meta
            .status
            .clone()
            .unwrap_or_else(|| default_status(row, meta).into()),
        root_cause: meta.root_cause.clone(),
        fix_target: meta.fix_target.clone(),
        plane: meta.plane.clone(),
        issue: meta.issue.clone(),
        owner: meta.owner.clone(),
        known_difference: meta.known_difference.clone(),
    })
}

/// True when the best backend is slower than OpenCV by more than 5 % and more than its own noise.
fn needs_work(item: &Item) -> bool {
    let noise = item
        .backends
        .iter()
        .find(|b| b.name == item.best_crate)
        .map_or(0.0, |b| b.noise);
    item.best_gap - 1.0 > MIN_GAP.max(noise)
}

/// Builds the queue: biggest best-backend gap first, ties broken by milliseconds lost.
pub fn build(bench: &Benchmarks, meta: &QueueMeta) -> Queue {
    let none = Meta::default();
    let (mut queue, at_parity): (Vec<Item>, Vec<Item>) = bench
        .benchmarks
        .iter()
        .filter_map(|r| item(r, meta.items.get(&r.id).unwrap_or(&none)))
        .partition(needs_work);
    queue.sort_by(|a, b| {
        b.best_gap
            .total_cmp(&a.best_gap)
            .then(b.ms_lost.total_cmp(&a.ms_lost))
    });
    for (i, it) in queue.iter_mut().enumerate() {
        it.rank = i + 1;
    }
    Queue { queue, at_parity }
}

fn linked(issue: &Option<String>) -> bool {
    issue
        .as_deref()
        .is_some_and(|u| u.starts_with(ISSUE_PREFIX) && u.contains("/issues/"))
}

fn item_problems(item: &Item) -> Vec<String> {
    let mut out = Vec::new();
    if !linked(&item.issue) {
        out.push(format!(
            "perf queue item {} ({:.2}x) has no GitHub issue link in catalog/perf_queue.yaml",
            item.id, item.best_gap
        ));
    }
    let ticket_ok = item
        .plane
        .as_deref()
        .is_some_and(|t| valid_ticket(t) || t.starts_with("RUSTCVGUID-"));
    if !ticket_ok {
        out.push(format!(
            "perf queue item {} has no valid Plane ticket",
            item.id
        ));
    }
    out
}

/// Rows whose backends do different work are project errors: fixed in the session that finds them, or
/// explained by a `known_difference` that links a GitHub issue.
fn parity_problems(bench: &Benchmarks, meta: &QueueMeta) -> Vec<String> {
    let explained = |id: &str| {
        meta.items
            .get(id)
            .is_some_and(|m| m.known_difference.is_some() && linked(&m.issue))
    };
    bench
        .benchmarks
        .iter()
        .filter(|r| r.parity == "mismatch" && !explained(&r.id))
        .map(|r| {
            format!(
                "benchmark {} compares different work (parity mismatch): fix it in the same session, or record a known_difference and a GitHub issue link in catalog/perf_queue.yaml",
                r.id
            )
        })
        .collect()
}

/// Every way the queue metadata disagrees with the benchmarks.
pub fn problems(bench: &Benchmarks, meta: &QueueMeta) -> Vec<String> {
    if bench.benchmarks.is_empty() {
        return Vec::new();
    }
    let queue = build(bench, meta);
    let mut out: Vec<String> = queue.queue.iter().flat_map(item_problems).collect();
    out.extend(parity_problems(bench, meta));
    let known: Vec<&str> = bench.benchmarks.iter().map(|r| r.id.as_str()).collect();
    out.extend(
        meta.items
            .keys()
            .filter(|id| !known.contains(&id.as_str()))
            .map(|id| format!("catalog/perf_queue.yaml lists unknown benchmark {id}")),
    );
    out
}

fn link(url: &Option<String>) -> String {
    match url {
        Some(u) => format!("[#{}]({u})", u.rsplit('/').next().unwrap_or("?")),
        None => "needs issue".into(),
    }
}

fn gaps(item: &Item) -> String {
    let cells: Vec<String> = item
        .backends
        .iter()
        .map(|b| match b.gap_opt {
            Some(o) => format!("{} {:.1}x ({:.1}x opt)", b.name, b.gap, o),
            None => format!("{} {:.1}x", b.name, b.gap),
        })
        .collect();
    cells.join("<br>")
}

fn queue_row(item: &Item) -> String {
    let owner = item
        .owner
        .as_deref()
        .map_or("unclaimed".to_string(), |o| format!("@{o}"));
    format!(
        "| {} | {} | {:.1}x | {} | {} | {} | {} | {} |",
        item.rank,
        item.operation.replace('|', "/"),
        item.best_gap,
        gaps(item),
        item.parity,
        item.status,
        link(&item.issue),
        owner
    )
}

fn parity_row(item: &Item) -> String {
    format!(
        "| {} | {:.2} ms | {} | {} |",
        item.operation.replace('|', "/"),
        item.opencv_ms,
        item.best_crate,
        gaps(item)
    )
}

const INTRO: &str = "Operations where the best Rust backend is still slower than OpenCV 5.1.0, biggest gap first. \
Gaps compare each crate against OpenCV timed with a freshly allocated output, and `opt` is the fat-LTO, native-CPU build. \
`probe-mismatch` means the backends do different work, so fix the benchmark before trusting the ratio.\n\n\
**Pick an item:** open its issue, say you are taking it, and set `owner` for it in `catalog/perf_queue.yaml` in your pull request. \
See `CONTRIBUTING.md` for how to run the benchmarks. The preferred place for a fix is `oxipix`, then a pull request to `imageproc` or `oximedia-cv`.\n";

fn crate_cell(item: &Item, name_prefix: &str) -> String {
    item.backends
        .iter()
        .find(|b| b.name.starts_with(name_prefix))
        .map_or("—".into(), |b| match b.gap_opt {
            Some(o) => format!("{:.1} ms, {:.1}x ({:.1}x opt)", b.ms, b.gap, o),
            None => format!("{:.1} ms, {:.1}x", b.ms, b.gap),
        })
}

/// Final per-crate table: every operation with each crate's time and gap, worst first, parity rows last.
pub fn summary(queue: &Queue) -> String {
    let mut lines = vec![
        "## Summary by crate\n".to_string(),
        "Every operation with each crate's median time and gap to OpenCV (fresh output buffer). `opt` is the fat-LTO, native-CPU build. The ranked work list is the [performance queue](perf-queue.md).\n".into(),
        "| Operation | OpenCV | image/imageproc | oximedia-cv / oxipix | Best gap |".into(),
        "|---|---|---|---|---|".into(),
    ];
    for it in queue.queue.iter().chain(&queue.at_parity) {
        let oxi = crate_cell(it, "oxi");
        lines.push(format!(
            "| {} | {:.3} ms | {} | {} | {:.1}x |",
            it.operation.replace('|', "/"),
            it.opencv_ms,
            crate_cell(it, "image"),
            oxi,
            it.best_gap
        ));
    }
    format!("{}\n", lines.join("\n"))
}

/// Markdown body shared by `PERF_QUEUE.md` and the website page.
pub fn markdown(queue: &Queue) -> String {
    let mut lines = vec![INTRO.to_string()];
    lines.push("| Rank | Operation | Best gap | Per-crate gap vs OpenCV | Parity | Status | Issue | Owner |".into());
    lines.push("|---|---|---|---|---|---|---|---|".into());
    lines.extend(queue.queue.iter().map(queue_row));
    let known: Vec<String> = queue
        .queue
        .iter()
        .chain(&queue.at_parity)
        .filter_map(|i| {
            Some(format!(
                "- **{}**: {}",
                i.operation.replace('|', "/"),
                i.known_difference.as_deref()?
            ))
        })
        .collect();
    if !known.is_empty() {
        lines.push("\n## Known differences\n".into());
        lines.push("Rows whose backends cannot run the same work, with the reason.\n".into());
        lines.extend(known);
    }
    lines.push("\n## At parity\n".into());
    lines.push("| Operation | OpenCV | Best crate | Per-crate gap |".into());
    lines.push("|---|---|---|---|".into());
    lines.extend(queue.at_parity.iter().map(parity_row));
    format!("{}\n", lines.join("\n"))
}

/// The files `cvg-tools generate` writes, as `(path relative to the repository root, content)`.
pub fn outputs(queue: &Queue) -> Vec<(String, String)> {
    let body = markdown(queue);
    let json = serde_json::to_string_pretty(queue).unwrap_or_default();
    vec![
        ("catalog/perf_queue.json".into(), format!("{json}\n")),
        (
            "PERF_QUEUE.md".into(),
            format!("# Performance priority queue\n\n{body}"),
        ),
        (
            "website/docs/perf-queue.md".into(),
            format!("---\ntitle: Performance priority queue\n---\n\n{body}"),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    fn row(id: &str, ocv: f64, rust: Option<f64>, oxi: Option<f64>, parity: &str) -> BenchRow {
        BenchRow {
            id: id.into(),
            tutorial: format!("tutorial_{id}"),
            operation: format!("{id} | op"),
            opencv_ms: Some(ocv),
            opencv_fresh_ms: Some(ocv),
            rust_ms: rust,
            oximedia_ms: oxi,
            parity: parity.into(),
            ..BenchRow::default()
        }
    }

    #[fixture]
    fn bench() -> Benchmarks {
        let mut noisy = row("noisy", 1.0, Some(1.1), None, "ok");
        noisy.rust_stats = Some(Stats {
            median: 1.1,
            mad: 0.1,
        });
        Benchmarks {
            benchmarks: vec![
                row("small", 1.0, Some(2.0), None, "ok"),
                row("big", 1.0, Some(50.0), Some(10.0), "mismatch"),
                row("tie_a", 1.0, Some(2.0), None, "n/a"),
                row("tie_big_ms", 10.0, Some(20.0), None, "ok"),
                row("parity", 1.0, Some(1.02), None, "ok"),
                noisy,
            ],
            ..Benchmarks::default()
        }
    }

    fn meta(entries: &[(&str, &str, &str)]) -> QueueMeta {
        let items = entries
            .iter()
            .map(|(id, plane, issue)| {
                let m = Meta {
                    plane: Some((*plane).into()),
                    issue: Some((*issue).into()),
                    ..Meta::default()
                };
                ((*id).to_string(), m)
            })
            .collect();
        QueueMeta { items }
    }

    #[rstest]
    fn queue_is_ranked_by_best_backend_then_ms_lost(bench: Benchmarks) {
        let q = build(&bench, &QueueMeta::default());
        let order: Vec<&str> = q.queue.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(order, ["big", "tie_big_ms", "small", "tie_a"]);
        assert_eq!(q.queue[0].rank, 1);
        assert_eq!(q.queue[0].best_crate, "oximedia-cv");
        assert_eq!(q.queue[0].best_gap, 10.0);
    }

    #[rstest]
    fn gaps_inside_five_percent_or_the_noise_are_at_parity(bench: Benchmarks) {
        let q = build(&bench, &QueueMeta::default());
        let parity: Vec<&str> = q.at_parity.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(parity, ["parity", "noisy"]);
    }

    #[rstest]
    fn mismatching_rows_default_to_probe_mismatch(bench: Benchmarks) {
        let q = build(&bench, &QueueMeta::default());
        assert_eq!(q.queue[0].status, "probe-mismatch");
        assert_eq!(q.queue[2].status, "unprobed");
    }

    #[rstest]
    fn rows_without_a_fair_opencv_time_are_skipped() {
        let mut b = Benchmarks::default();
        let mut r = row("none", 1.0, Some(5.0), None, "ok");
        r.opencv_ms = None;
        r.opencv_fresh_ms = None;
        b.benchmarks = vec![r, row("no_backend", 1.0, None, None, "ok")];
        let q = build(&b, &QueueMeta::default());
        assert!(q.queue.is_empty() && q.at_parity.is_empty());
    }

    #[rstest]
    fn every_queue_item_needs_an_issue_and_a_ticket(bench: Benchmarks) {
        let found = problems(&bench, &QueueMeta::default());
        assert_eq!(
            found
                .iter()
                .filter(|p| p.contains("no GitHub issue"))
                .count(),
            4
        );
        assert_eq!(
            found.iter().filter(|p| p.contains("Plane ticket")).count(),
            4
        );
    }

    #[rstest]
    #[case("https://example.com/issues/1", true)]
    #[case("https://github.com/o/r/pull/1", true)]
    #[case("https://github.com/o/r/issues/1", false)]
    fn only_github_issue_urls_count_as_links(
        #[case] url: &str,
        #[case] flagged: bool,
        bench: Benchmarks,
    ) {
        let m = meta(&[("big", "RUSTCVGUID-9", url)]);
        let found = problems(&bench, &m);
        assert_eq!(
            found
                .iter()
                .any(|p| p.contains("item big (") && p.contains("GitHub issue")),
            flagged
        );
    }

    #[rstest]
    fn unknown_meta_ids_are_reported_and_empty_benchmarks_pass(bench: Benchmarks) {
        let m = meta(&[("ghost", "OXI-1", "https://github.com/o/r/issues/1")]);
        assert!(
            problems(&bench, &m)
                .iter()
                .any(|p| p.contains("unknown benchmark ghost"))
        );
        assert!(problems(&Benchmarks::default(), &m).is_empty());
    }

    #[rstest]
    fn markdown_links_issues_and_summary_lists_every_crate(bench: Benchmarks) {
        let mut m = meta(&[("big", "OXI-7", "https://github.com/o/r/issues/12")]);
        m.items.get_mut("big").unwrap().owner = Some("alice".into());
        let q = build(&bench, &m);
        let md = markdown(&q);
        assert!(md.contains("[#12](https://github.com/o/r/issues/12)") && md.contains("@alice"));
        assert!(
            md.contains("needs issue") && md.contains("unclaimed") && md.contains("## At parity")
        );
        let table = summary(&q);
        assert!(
            table.contains("image/imageproc")
                && table.contains("2.0 ms, 2.0x")
                && table.contains("—")
        );
    }

    #[rstest]
    fn outputs_cover_json_root_and_site(bench: Benchmarks) {
        let files = outputs(&build(&bench, &QueueMeta::default()));
        let paths: Vec<&str> = files.iter().map(|(p, _)| p.as_str()).collect();
        assert_eq!(
            paths,
            [
                "catalog/perf_queue.json",
                "PERF_QUEUE.md",
                "website/docs/perf-queue.md"
            ]
        );
        assert!(files[2].1.starts_with("---\ntitle:"));
    }

    fn mismatching(id: &str) -> Benchmarks {
        Benchmarks {
            benchmarks: vec![row(id, 1.0, Some(1.0), None, "mismatch")],
            ..Benchmarks::default()
        }
    }

    #[rstest]
    #[case(None, None, true)]
    #[case(Some("upstream defect"), None, true)]
    #[case(
        Some("upstream defect"),
        Some("https://github.com/o/r/issues/3"),
        false
    )]
    fn a_parity_mismatch_needs_a_known_difference_and_an_issue(
        #[case] reason: Option<&str>,
        #[case] issue: Option<&str>,
        #[case] flagged: bool,
    ) {
        let mut m = QueueMeta::default();
        m.items.insert(
            "m".into(),
            Meta {
                known_difference: reason.map(str::to_string),
                issue: issue.map(str::to_string),
                ..Meta::default()
            },
        );
        let found = problems(&mismatching("m"), &m);
        assert_eq!(found.iter().any(|p| p.contains("parity mismatch")), flagged);
    }

    #[rstest]
    fn known_differences_are_listed_and_change_the_status() {
        let mut b = mismatching("m");
        b.benchmarks[0].rust_ms = Some(5.0);
        let mut m = QueueMeta::default();
        m.items.insert(
            "m".into(),
            Meta {
                known_difference: Some("does less work".into()),
                ..Meta::default()
            },
        );
        let q = build(&b, &m);
        assert_eq!(q.queue[0].status, "known-difference");
        let md = markdown(&q);
        assert!(md.contains("## Known differences") && md.contains("does less work"));
        assert!(!markdown(&build(&b, &QueueMeta::default())).contains("## Known differences"));
    }

    #[rstest]
    fn optimised_gaps_are_reported_beside_default_ones() {
        let mut r = row("opt", 1.0, Some(4.0), None, "ok");
        r.rust_opt_ms = Some(3.0);
        let b = Benchmarks {
            benchmarks: vec![r],
            ..Benchmarks::default()
        };
        let q = build(&b, &QueueMeta::default());
        assert_eq!(q.queue[0].best_gap, 3.0);
        assert!(markdown(&q).contains("4.0x (3.0x opt)"));
        assert!(summary(&q).contains("4.0 ms, 4.0x (3.0x opt)"));
    }
}
