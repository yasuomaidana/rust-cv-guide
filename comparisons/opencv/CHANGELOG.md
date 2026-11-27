# Changelog

## Unreleased

- Initial OpenCV 5.1.0 vs Pure Rust (`image`/`imageproc`) vs `oximedia-cv`/`oxipix` release benchmark runner.
- [RUSTCVGUID-156] Fair-probe harness: `run.sh` measures a default and an optimised (fat LTO, one codegen unit, `target-cpu=native`) Rust build; OpenCV is timed with a reused and a freshly allocated output; adaptive iteration counts (>= 200 ms per series) with median, p10, p90 and MAD; calibration rows (empty closure, allocate-and-drop per input size); parity digests (size and mean, or element counts) flag rows where the backends do different work; OpenCV HAL, SIMD and threading build lines are recorded.
- [RUSTCVGUID-184] Parity digests: shift inliers (h = 2) beside element counts (h = 1); feature_description runs FAST + BRIEF on every side (OpenCV via xfeatures2d); canny uses the L2 gradient, laplace the same normalisation and Hough lines the same suppression as the Rust snippets.
