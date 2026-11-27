# Contributing

## Performance work

The [performance queue](PERF_QUEUE.md) lists every operation where the best Rust backend is still slower than OpenCV 5.1.0, biggest gap first. Each item links to a public GitHub issue labelled `perf-gap`.

1. **Pick an item** from `PERF_QUEUE.md` and comment on its issue.
2. **Claim it** by setting `owner` to your GitHub user for that item in `catalog/perf_queue.yaml` in your pull request.
3. **Reproduce the numbers**: `brew install opencv`, then `comparisons/opencv/run.sh`. It builds the native benchmark, measures a default and an optimised Rust build, and rewrites `catalog/benchmarks.json`.
4. **Find the cause** before changing code. Compare against the OpenCV 5.1.0 source (and the KleidiCV / carotene hardware layer on ARM). A gap that survives the optimised build is algorithmic or a SIMD kernel, not a compiler setting.
5. **Fix it where it helps most**: `oxipix` first, then a pull request to `imageproc` or `oximedia-cv`. A new crate is only worth adding if it clearly saves development time, and it must be benchmarked as its own backend.
6. **Refresh the generated files**: `cargo run -p cvg-tools -- generate`, then `cargo run -p cvg-tools -- check`. The check fails if a queue item has no issue link, or if `PERF_QUEUE.md` and `catalog/perf_queue.json` no longer match the benchmarks.

A row where `parity` is `mismatch` compares different work (different output size, mean, or element count). Fix the benchmark first, because its ratio cannot be trusted.

New tutorials need a benchmark row. If the best Rust backend is more than 5% slower than OpenCV (and more than the measured noise), `cvg-tools check` requires a queue entry with an issue link.
