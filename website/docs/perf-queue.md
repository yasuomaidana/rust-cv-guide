---
title: Performance priority queue
---

Operations where the best Rust backend is still slower than OpenCV 5.1.0, biggest gap first. Gaps compare each crate against OpenCV timed with a freshly allocated output, and `opt` is the fat-LTO, native-CPU build. `probe-mismatch` means the backends do different work, so fix the benchmark before trusting the ratio.

**Pick an item:** open its issue, say you are taking it, and set `owner` for it in `catalog/perf_queue.yaml` in your pull request. See `CONTRIBUTING.md` for how to run the benchmarks. The preferred place for a fix is `oxipix`, then a pull request to `imageproc` or `oximedia-cv`.

| Rank | Operation | Best gap | Per-crate gap vs OpenCV | Parity | Status | Issue | Owner |
|---|---|---|---|---|---|---|---|
| 1 | Template match (TM_SQDIFF) | 75.3x | image/imageproc 75.3x (75.5x opt) | n/a | unprobed | [#39](https://github.com/yasuomaidana/rust-cv-guide/issues/39) | unclaimed |
| 2 | Median filter (5x5, r=2) | 41.3x | image/imageproc 41.3x (42.8x opt)<br>oximedia-cv 88.8x (88.7x opt) | ok | unprobed | [#40](https://github.com/yasuomaidana/rust-cv-guide/issues/40) | unclaimed |
| 3 | Erode (5x5 square, r=2) | 28.1x | image/imageproc 28.1x (30.0x opt)<br>oximedia-cv 331.8x (281.1x opt) | ok | unprobed | [#41](https://github.com/yasuomaidana/rust-cv-guide/issues/41) | unclaimed |
| 4 | 3-level pyrDown | 19.4x | image/imageproc 41.5x (44.0x opt)<br>oximedia-cv 19.6x (19.4x opt) | ok | unprobed | [#42](https://github.com/yasuomaidana/rust-cv-guide/issues/42) | unclaimed |
| 5 | Canny + circle Hough (r=15..40) | 16.2x | oximedia-cv 16.4x (16.2x opt) | n/a | unprobed | [#43](https://github.com/yasuomaidana/rust-cv-guide/issues/43) | unclaimed |
| 6 | Top-hat morphology (7x7, r=3) | 14.7x | image/imageproc 14.7x (39.1x opt)<br>oximedia-cv 481.8x (374.8x opt) | ok | unprobed | [#44](https://github.com/yasuomaidana/rust-cv-guide/issues/44) | unclaimed |
| 7 | Gaussian blur (sigma=1.1) | 9.1x | image/imageproc 9.1x (9.4x opt)<br>oximedia-cv 23.7x (24.6x opt) | ok | unprobed | [#46](https://github.com/yasuomaidana/rust-cv-guide/issues/46) | unclaimed |
| 8 | Bilateral filter (d=5, r=2, sc=30) | 7.3x | image/imageproc 11.3x (7.3x opt)<br>oximedia-cv 41.0x (42.9x opt) | ok | unprobed | [#45](https://github.com/yasuomaidana/rust-cv-guide/issues/45) | unclaimed |
| 9 | Perspective unwarp (4-point quad) | 7.1x | image/imageproc 7.5x (7.1x opt)<br>oxipix 11.6x (10.8x opt) | ok | unprobed | [#48](https://github.com/yasuomaidana/rust-cv-guide/issues/48) | unclaimed |
| 10 | Gaussian blur + Canny (50/150) | 6.9x | image/imageproc 7.6x (6.9x opt)<br>oximedia-cv 9.7x (8.6x opt) | ok | unprobed | [#47](https://github.com/yasuomaidana/rust-cv-guide/issues/47) | unclaimed |
| 11 | Rotate 30 deg + scale 0.6 | 6.3x | image/imageproc 10.6x (9.9x opt)<br>oximedia-cv 7.0x (6.3x opt) | ok | unprobed | [#49](https://github.com/yasuomaidana/rust-cv-guide/issues/49) | unclaimed |
| 12 | addWeighted (0.5 / 0.5) | 5.8x | image/imageproc 7.0x (5.8x opt)<br>oximedia-cv 13.5x (12.3x opt) | n/a | unprobed | [#50](https://github.com/yasuomaidana/rust-cv-guide/issues/50) | unclaimed |
| 13 | Box blur (5x5, r=2) | 5.7x | image/imageproc 5.7x (6.2x opt)<br>oximedia-cv 6.4x (6.5x opt) | ok | unprobed | [#51](https://github.com/yasuomaidana/rust-cv-guide/issues/51) | unclaimed |
| 14 | FAST-9 corners (t=30, 3x3 non-max suppression, uncapped) | 5.6x | image/imageproc 6.2x (5.6x opt)<br>oximedia-cv 13.4x (12.6x opt) | mismatch | known-difference | [#56](https://github.com/yasuomaidana/rust-cv-guide/issues/56) | unclaimed |
| 15 | Sobel X derivative | 5.5x | image/imageproc 27.1x (27.2x opt)<br>oximedia-cv 5.5x (5.5x opt) | ok | unprobed | [#52](https://github.com/yasuomaidana/rust-cv-guide/issues/52) | unclaimed |
| 16 | Contrast + brightness (a=1.3, b=40) | 5.4x | image/imageproc 6.8x (5.4x opt) | ok | unprobed | [#53](https://github.com/yasuomaidana/rust-cv-guide/issues/53) | unclaimed |
| 17 | FAST + BRIEF matching (t=20, 250 kp, up to 32 bits) | 4.9x | image/imageproc 5.2x (4.9x opt)<br>oximedia-cv 6.9x (6.8x opt) | ok | unprobed | [#66](https://github.com/yasuomaidana/rust-cv-guide/issues/66) | unclaimed |
| 18 | 2D convolution (5x5 box kernel) | 4.3x | image/imageproc 5.1x (4.3x opt)<br>oximedia-cv 12.6x (12.8x opt) | ok | unprobed | [#55](https://github.com/yasuomaidana/rust-cv-guide/issues/55) | unclaimed |
| 19 | Distance transform (L2) | 3.3x | image/imageproc 3.3x (3.4x opt) | n/a | unprobed | [#57](https://github.com/yasuomaidana/rust-cv-guide/issues/57) | unclaimed |
| 20 | Canny + standard Hough lines | 3.2x | image/imageproc 3.5x (3.2x opt)<br>oximedia-cv 6.5x (6.3x opt) | ok | unprobed | [#60](https://github.com/yasuomaidana/rust-cv-guide/issues/60) | unclaimed |
| 21 | PSNR | 2.7x | image/imageproc 2.7x (2.7x opt)<br>oximedia-cv 2.8x (2.8x opt) | n/a | unprobed | [#58](https://github.com/yasuomaidana/rust-cv-guide/issues/58) | unclaimed |
| 22 | Gaussian blur + Laplacian edge | 2.6x | image/imageproc 4.7x (2.6x opt)<br>oximedia-cv 7.8x (7.9x opt) | ok | unprobed | [#54](https://github.com/yasuomaidana/rust-cv-guide/issues/54) | unclaimed |
| 23 | SSIM (11x11 Gaussian) | 2.0x | image/imageproc 2.0x (2.0x opt)<br>oximedia-cv 35.2x (30.4x opt) | n/a | unprobed | [#59](https://github.com/yasuomaidana/rust-cv-guide/issues/59) | unclaimed |
| 24 | Find contours + approxPolyDP | 1.6x | image/imageproc 3.3x (3.0x opt)<br>oxipix 1.7x (1.6x opt) | n/a | unprobed | [#61](https://github.com/yasuomaidana/rust-cv-guide/issues/61) | unclaimed |
| 25 | Otsu binary threshold | 1.5x | image/imageproc 1.5x (1.5x opt)<br>oxipix 5.2x (4.5x opt) | ok | unprobed | [#62](https://github.com/yasuomaidana/rust-cv-guide/issues/62) | unclaimed |
| 26 | HSV red range mask | 1.5x | image/imageproc 1.6x (1.5x opt)<br>oximedia-cv 3.6x (3.0x opt) | ok | unprobed | [#63](https://github.com/yasuomaidana/rust-cv-guide/issues/63) | unclaimed |
| 27 | Histogram equalisation | 1.2x | image/imageproc 1.4x (1.4x opt)<br>oximedia-cv 1.2x (1.2x opt) | ok | unprobed | [#64](https://github.com/yasuomaidana/rust-cv-guide/issues/64) | unclaimed |

## Known differences

Rows whose backends cannot run the same work, with the reason.

- **FAST-9 corners (t=30, 3x3 non-max suppression, uncapped)**: oximedia-cv FastDetector counts the 9 contiguous ring pixels linearly over indices 0 to 15 and never detects an arc that wraps around the ring, so it returns 1424 corners where FAST-9 (OpenCV, imageproc) returns 2091 and 2204. The oximedia column is therefore not a full FAST-9; reported upstream as https://github.com/cool-japan/oximedia/issues/32.

## At parity

| Operation | OpenCV | Best crate | Per-crate gap |
|---|---|---|---|
| Oriented keypoints (t=30, max=250) | 6.54 ms | image/imageproc | image/imageproc 0.3x (0.3x opt)<br>oximedia-cv 1.1x (1.1x opt) |
