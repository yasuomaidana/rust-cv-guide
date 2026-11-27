# Performance priority queue

Operations where the best Rust backend is still slower than OpenCV 5.1.0, biggest gap first. Gaps compare each crate against OpenCV timed with a freshly allocated output, and `opt` is the fat-LTO, native-CPU build. `probe-mismatch` means the backends do different work, so fix the benchmark before trusting the ratio.

**Pick an item:** open its issue, say you are taking it, and set `owner` for it in `catalog/perf_queue.yaml` in your pull request. See `CONTRIBUTING.md` for how to run the benchmarks. The preferred place for a fix is `oxipix`, then a pull request to `imageproc` or `oximedia-cv`.

| Rank | Operation | Best gap | Per-crate gap vs OpenCV | Parity | Status | Issue | Owner |
|---|---|---|---|---|---|---|---|
| 1 | Template match (TM_SQDIFF) | 74.0x | image/imageproc 74.0x (76.0x opt) | n/a | unprobed | [#39](https://github.com/yasuomaidana/rust-cv-guide/issues/39) | unclaimed |
| 2 | Median filter (5x5, r=2) | 42.4x | image/imageproc 43.0x (42.4x opt)<br>oximedia-cv 89.9x (116.5x opt) | ok | unprobed | [#40](https://github.com/yasuomaidana/rust-cv-guide/issues/40) | unclaimed |
| 3 | Erode (5x5 square, r=2) | 27.6x | image/imageproc 27.6x (29.3x opt)<br>oximedia-cv 333.7x (278.1x opt) | ok | unprobed | [#41](https://github.com/yasuomaidana/rust-cv-guide/issues/41) | unclaimed |
| 4 | 3-level pyrDown | 18.8x | image/imageproc 40.5x (40.9x opt)<br>oximedia-cv 19.6x (18.8x opt) | ok | unprobed | [#42](https://github.com/yasuomaidana/rust-cv-guide/issues/42) | unclaimed |
| 5 | Canny + circle Hough (r=15..40) | 15.8x | oximedia-cv 16.0x (15.8x opt) | n/a | unprobed | [#43](https://github.com/yasuomaidana/rust-cv-guide/issues/43) | unclaimed |
| 6 | Top-hat morphology (7x7, r=3) | 14.9x | image/imageproc 14.9x (40.5x opt)<br>oximedia-cv 509.6x (386.0x opt) | ok | unprobed | [#44](https://github.com/yasuomaidana/rust-cv-guide/issues/44) | unclaimed |
| 7 | Bilateral filter (d=5, r=2, sc=30) | 10.2x | image/imageproc 11.4x (10.2x opt)<br>oximedia-cv 41.0x (43.9x opt) | ok | unprobed | [#45](https://github.com/yasuomaidana/rust-cv-guide/issues/45) | unclaimed |
| 8 | Gaussian blur (sigma=1.1) | 8.8x | image/imageproc 8.8x (9.1x opt)<br>oximedia-cv 24.0x (24.1x opt) | ok | unprobed | [#46](https://github.com/yasuomaidana/rust-cv-guide/issues/46) | unclaimed |
| 9 | Gaussian blur + Canny (50/150) | 7.4x | image/imageproc 7.9x (7.4x opt)<br>oximedia-cv 8.6x (8.3x opt) | mismatch | probe-mismatch | [#47](https://github.com/yasuomaidana/rust-cv-guide/issues/47) | unclaimed |
| 10 | Perspective unwarp (4-point quad) | 7.0x | image/imageproc 7.2x (7.0x opt)<br>oxipix 12.2x (10.7x opt) | ok | unprobed | [#48](https://github.com/yasuomaidana/rust-cv-guide/issues/48) | unclaimed |
| 11 | Rotate 30 deg + scale 0.6 | 6.0x | image/imageproc 10.1x (9.3x opt)<br>oximedia-cv 6.8x (6.0x opt) | ok | unprobed | [#49](https://github.com/yasuomaidana/rust-cv-guide/issues/49) | unclaimed |
| 12 | addWeighted (0.5 / 0.5) | 5.7x | image/imageproc 6.7x (5.7x opt)<br>oximedia-cv 13.4x (12.2x opt) | n/a | unprobed | [#50](https://github.com/yasuomaidana/rust-cv-guide/issues/50) | unclaimed |
| 13 | Box blur (5x5, r=2) | 5.7x | image/imageproc 5.7x (5.7x opt)<br>oximedia-cv 6.4x (6.5x opt) | ok | unprobed | [#51](https://github.com/yasuomaidana/rust-cv-guide/issues/51) | unclaimed |
| 14 | Sobel X derivative | 5.4x | image/imageproc 27.1x (27.4x opt)<br>oximedia-cv 5.6x (5.4x opt) | ok | unprobed | [#52](https://github.com/yasuomaidana/rust-cv-guide/issues/52) | unclaimed |
| 15 | Contrast + brightness (a=1.3, b=40) | 5.3x | image/imageproc 7.1x (5.3x opt) | ok | unprobed | [#53](https://github.com/yasuomaidana/rust-cv-guide/issues/53) | unclaimed |
| 16 | Gaussian blur + Laplacian edge | 5.1x | image/imageproc 5.1x (5.4x opt)<br>oximedia-cv 8.4x (9.6x opt) | mismatch | probe-mismatch | [#54](https://github.com/yasuomaidana/rust-cv-guide/issues/54) | unclaimed |
| 17 | 2D convolution (5x5 box kernel) | 4.1x | image/imageproc 5.1x (4.1x opt)<br>oximedia-cv 12.6x (14.5x opt) | ok | unprobed | [#55](https://github.com/yasuomaidana/rust-cv-guide/issues/55) | unclaimed |
| 18 | FAST-9 keypoints (t=30, max=250) | 3.9x | image/imageproc 4.2x (3.9x opt)<br>oximedia-cv 12.5x (12.2x opt) | mismatch | probe-mismatch | [#56](https://github.com/yasuomaidana/rust-cv-guide/issues/56) | unclaimed |
| 19 | Distance transform (L2) | 3.3x | image/imageproc 3.3x (3.8x opt) | n/a | unprobed | [#57](https://github.com/yasuomaidana/rust-cv-guide/issues/57) | unclaimed |
| 20 | PSNR | 2.6x | image/imageproc 2.7x (2.6x opt)<br>oximedia-cv 2.8x (2.7x opt) | n/a | unprobed | [#58](https://github.com/yasuomaidana/rust-cv-guide/issues/58) | unclaimed |
| 21 | SSIM (11x11 Gaussian) | 2.0x | image/imageproc 2.1x (2.0x opt)<br>oximedia-cv 32.9x (28.4x opt) | n/a | unprobed | [#59](https://github.com/yasuomaidana/rust-cv-guide/issues/59) | unclaimed |
| 22 | Canny + standard Hough lines | 1.7x | image/imageproc 1.9x (1.7x opt)<br>oximedia-cv 3.6x (3.3x opt) | mismatch | probe-mismatch | [#60](https://github.com/yasuomaidana/rust-cv-guide/issues/60) | unclaimed |
| 23 | Find contours + approxPolyDP | 1.6x | image/imageproc 3.3x (3.0x opt)<br>oxipix 1.7x (1.6x opt) | n/a | unprobed | [#61](https://github.com/yasuomaidana/rust-cv-guide/issues/61) | unclaimed |
| 24 | Otsu binary threshold | 1.5x | image/imageproc 1.5x (1.5x opt)<br>oxipix 5.3x (4.6x opt) | ok | unprobed | [#62](https://github.com/yasuomaidana/rust-cv-guide/issues/62) | unclaimed |
| 25 | HSV red range mask | 1.4x | image/imageproc 1.5x (1.4x opt)<br>oximedia-cv 3.6x (2.9x opt) | ok | unprobed | [#63](https://github.com/yasuomaidana/rust-cv-guide/issues/63) | unclaimed |
| 26 | Histogram equalisation | 1.2x | image/imageproc 1.4x (1.4x opt)<br>oximedia-cv 1.2x (1.2x opt) | ok | unprobed | [#64](https://github.com/yasuomaidana/rust-cv-guide/issues/64) | unclaimed |

## At parity

| Operation | OpenCV | Best crate | Per-crate gap |
|---|---|---|---|
| ORB keypoints (t=30, max=250) | 6.70 ms | image/imageproc | image/imageproc 0.3x (0.3x opt)<br>oximedia-cv 0.6x (0.5x opt) |
| Binary feature matching (250 kp) | 4.33 ms | oximedia-cv | image/imageproc 0.9x (0.9x opt)<br>oximedia-cv 0.6x (0.5x opt) |
