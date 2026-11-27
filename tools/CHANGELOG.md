# Changelog

## Unreleased

- Per-section coverage files, guide bodies and `{{snippet}}` embedding, evidence rules in `check`.
- Catalog check and page generator.
- [RUSTCVGUID-184] `known_difference` in `catalog/perf_queue.yaml`; `cvg-tools check` fails on a benchmark row with `parity: mismatch` that has none (or no issue link); known differences are listed in `PERF_QUEUE.md`.
- [RUSTCVGUID-186] `root_cause` is required for queue items; `PERF_QUEUE.md` lists "Why it is slower, and the fix".
