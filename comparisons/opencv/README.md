# OpenCV 5.1.0 vs Pure Rust vs oximedia/oxipix Benchmark Harness

Standalone runner (excluded from the root workspace so CI does not require native OpenCV) that executes:

1. `bench.cpp` compiled with `-O3` against Homebrew OpenCV 5.1.0 (`opencv5`).
2. `src/main.rs` compiled with `--release` calling the exact functions from `snippets/examples/`.

It writes measured median wall-clock timings (excluding image decode and fixture setup) to `catalog/benchmarks.json`.

## Running

```bash
./run.sh            # needs Homebrew OpenCV 5.1 (`brew install opencv`); writes ../../catalog/benchmarks.json
```

`run.sh` builds `bench.cpp` with `-O3`, runs the Rust runner twice (default release, then fat LTO + one codegen unit + `target-cpu=native`) and merges both into the catalog.

## What the numbers mean

- `opencv_ms` reuses one output `Mat` (the OpenCV idiom); `opencv_fresh_ms` releases it before each call, so it allocates like the Rust snippets do. Compare Rust against `opencv_fresh_ms`.
- `rust_opt_ms` / `oximedia_opt_ms` come from the optimised build. A gap that survives it is not a build-profile effect.
- `calibration` is the probe-overhead floor: an empty closure and one allocate-and-drop per input size.
- `rust_stats` / `oximedia_stats` hold median, p10, p90, MAD and iteration count. A ratio only counts as a difference when it exceeds 5 % and twice the relative MAD.
- `parity` is `mismatch` when the backends produce outputs that differ in size or mean (images) or by more than 25 % in count (keypoints, lines). A mismatching row compares different work and must be fixed before its ratio is trusted.

## The oxipix imgproc backend

`src/next.rs` runs the `oxipix` `imgproc` kernels (OXI-94, OXI-95, OXI-96; not released yet, so `Cargo.toml` depends on their git branch) on the same input and with the same parameters as each tutorial snippet. Rows carry `oxipix_next_ms` and friends, and the parity check covers the new backend like the others. After oxipix publishes the kernels, replace the git dependency with the released version.
