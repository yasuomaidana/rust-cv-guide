# Changelog

## Unreleased

- Scaffold: Docusaurus site, CI and GitHub Pages deploy.
- [RUSTCVGUID-157] Public performance queue: `cvg-tools generate` writes `catalog/perf_queue.json`, `PERF_QUEUE.md` and `website/docs/perf-queue.md` from the benchmarks, ranked by the best Rust backend's gap to OpenCV with per-crate gaps; each item links to a `perf-gap` GitHub issue (hand-kept in `catalog/perf_queue.yaml`); `cvg-tools check` fails on a missing issue link or a stale queue file; per-crate summary table on the benchmarks page; issue template and `CONTRIBUTING.md`.
- [RUSTCVGUID-184] Feature tutorials run one pipeline on every backend: FAST-9 with 3x3 non-max suppression (imageproc's `corners_fast9` has none), a shared 256-test 31x31 BRIEF (`cvg_snippets::brief`, bit-for-bit what `imageproc` computes) and Hamming matching; the oximedia column uses `FastDetector` instead of `OrbExtractor`, which is a scale-space (SIFT) detector, not ORB. imageproc's canny snippet no longer blurs twice, the Hough snippets take a suppression radius, and the benchmark compares them with OpenCV settings that match. `cvg-tools check` fails on parity mismatches without a recorded known difference; benchmark audit rules added to AGENTS.md.
- [RUSTCVGUID-186] Root cause and fix target for every queue item (`root_cause`, `fix_target`, status `researched`), shown in `PERF_QUEUE.md`; `cvg-tools check` fails on a queue item without a root cause.
- [RUSTCVGUID-187] The harness benchmarks the oxipix `imgproc` kernels (git dependency on the OXI-96 branch, `src/next.rs`) as a fourth backend beside imageproc, oximedia-cv and OpenCV; rows keep parity against OpenCV for it. Queue items with an oxipix kernel move to `fix-proposed` and link their oxipix pull request (yasuomaidana/oxipix#5, #6, #7).
