# Changelog

## Unreleased

- Per-section coverage files, guide bodies and `{{snippet}}` embedding, evidence rules in `check`.
- Catalog check and page generator.
- [RUSTCVGUID-184] `known_difference` in `catalog/perf_queue.yaml`; `cvg-tools check` fails on a benchmark row with `parity: mismatch` that has none (or no issue link); known differences are listed in `PERF_QUEUE.md`.
- [RUSTCVGUID-186] `root_cause` is required for queue items; `PERF_QUEUE.md` lists "Why it is slower, and the fix".
- [RUSTCVGUID-187] `oxipix_next_*` row fields, the `oxipix (imgproc)` backend in the queue ranking and an "oxipix imgproc" column in the summary table.
- [RUSTCVGUID-188] `sweep` module: renders the "Threads and picture size" page from the catalog `size_sweep`.
- [RUSTCVGUID-188] Sweep page: "oxipix, all threads" column and the best-Rust-on-any-threads gap.
- [RUSTCVGUID-189] `choose` module (recommendation page); native `opencv_*` examples are proven by `snippets-opencv` (manifest plus tests) instead of the live crate.
