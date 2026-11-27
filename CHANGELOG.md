# Changelog

## Unreleased

- Scaffold: Docusaurus site, CI and GitHub Pages deploy.
- [RUSTCVGUID-157] Public performance queue: `cvg-tools generate` writes `catalog/perf_queue.json`, `PERF_QUEUE.md` and `website/docs/perf-queue.md` from the benchmarks, ranked by the best Rust backend's gap to OpenCV with per-crate gaps; each item links to a `perf-gap` GitHub issue (hand-kept in `catalog/perf_queue.yaml`); `cvg-tools check` fails on a missing issue link or a stale queue file; per-crate summary table on the benchmarks page; issue template and `CONTRIBUTING.md`.
