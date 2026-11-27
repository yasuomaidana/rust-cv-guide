---
title: Rust only, OpenCV, or both?
---

The guide shows each tutorial three ways: Rust only (`image` + `imageproc`, or `oxipix` / `oximedia-cv`) and the `opencv` crate, which calls a native OpenCV. This page says, from the benchmarks, where staying in Rust costs little and where OpenCV is worth the dependency. The `opencv` column is what a Rust program gets from the guide's `opencv` example on all threads, including the copies between `image` buffers and OpenCV matrices. The Rust column is the fastest Rust backend measured, including `oxipix` with its `parallel` feature.

The rule: within 1.5x of OpenCV, or less than 0.2 ms apart, stay in Rust; up to 4x, stay in Rust unless this step is your bottleneck; beyond that, call OpenCV for it.

| Operation | Picture | Fastest Rust | `opencv` crate | Rust vs OpenCV | Recommendation |
|---|---|---|---|---|---|
| addWeighted (0.5 / 0.5) | 320x240 RGB | 0.06 ms (oxipix) | 0.04 ms | 1.5x | Rust only |
| Contrast + brightness (a=1.3, b=40) | 600x400 RGB | 0.14 ms (oxipix) | 0.06 ms | 2.4x | Rust only |
| Otsu binary threshold | 384x303 grey | 0.04 ms (oxipix) | 0.04 ms | 0.9x | Rust only |
| Histogram equalisation | 640x480 grey | 0.32 ms (oxipix) | 0.53 ms | 0.6x | Rust only |
| Erode (5x5 square, r=2) | 384x303 grey | 0.04 ms (oxipix) | 0.01 ms | 2.6x | Rust only |
| Box blur (5x5, r=2) | 451x300 RGB | 0.11 ms (oxipix, threads) | 0.25 ms | 0.5x | Rust only |
| Gaussian blur (sigma=1.1) | 451x300 RGB | 0.13 ms (oxipix, threads) | 0.20 ms | 0.6x | Rust only |
| Median filter (5x5, r=2) | 451x300 RGB | 3.16 ms (oxipix) | 0.13 ms | 23.7x | Mixed approach: call OpenCV for this step |
| Bilateral filter (d=5, r=2, sc=30) | 451x300 RGB | 0.68 ms (oxipix, threads) | 0.45 ms | 1.5x | Rust first; call OpenCV for this step if it is your bottleneck |
| Gaussian blur + Laplacian edge | 640x480 grey | 0.65 ms (oxipix, threads) | 0.51 ms | 1.3x | Rust only |
| Sobel X derivative | 640x480 grey | 0.10 ms (oxipix) | 0.16 ms | 0.6x | Rust only |
| Gaussian blur + Canny (50/150) | 640x480 grey | 1.71 ms (oxipix, threads) | 0.69 ms | 2.5x | Rust first; call OpenCV for this step if it is your bottleneck |
| 3-level pyrDown | 451x300 RGB | 0.14 ms (oxipix) | 0.05 ms | 2.5x | Rust only |
| Rotate 30 deg + scale 0.6 | 512x512 RGB | 0.26 ms (oxipix, threads) | 0.12 ms | 2.1x | Rust only |
| Template match (TM_SQDIFF) | 451x300 vs 40x40 grey | 2.08 ms (oxipix, threads) | 0.89 ms | 2.3x | Rust first; call OpenCV for this step if it is your bottleneck |
| Gaussian blur (sigma=1.1, RGB) | small (640x480) | 0.24 ms (oxipix, threads) | 0.38 ms | 0.6x | Rust only |
| Sobel x (grey) | small (640x480) | 0.06 ms (oxipix) | 0.10 ms | 0.6x | Rust only |
| Canny 50/150 (grey) | small (640x480) | 1.02 ms (oxipix, threads) | 0.47 ms | 2.2x | Rust first; call OpenCV for this step if it is your bottleneck |
| Median 5x5 (RGB) | small (640x480) | 2.39 ms (oxipix, threads) | 0.23 ms | 10.2x | Mixed approach: call OpenCV for this step |
| Erode 5x5 (grey) | small (640x480) | 0.08 ms (oxipix) | 0.04 ms | 2.2x | Rust only |
| Bilateral r=2 (RGB) | small (640x480) | 1.41 ms (oxipix, threads) | 0.69 ms | 2.0x | Rust first; call OpenCV for this step if it is your bottleneck |
| Rotate 30 deg, scale 0.6 (RGB) | small (640x480) | 0.29 ms (oxipix, threads) | 0.14 ms | 2.1x | Rust only |
| pyrDown (RGB) | small (640x480) | 0.22 ms (oxipix) | 0.06 ms | 3.8x | Rust only |
| Gaussian blur (sigma=1.1, RGB) | medium (1920x1080) | 1.02 ms (oxipix, threads) | 1.96 ms | 0.5x | Rust only |
| Sobel x (grey) | medium (1920x1080) | 0.36 ms (oxipix) | 0.46 ms | 0.8x | Rust only |
| Canny 50/150 (grey) | medium (1920x1080) | 5.13 ms (oxipix, threads) | 1.79 ms | 2.9x | Rust first; call OpenCV for this step if it is your bottleneck |
| Median 5x5 (RGB) | medium (1920x1080) | 14.79 ms (oxipix, threads) | 1.37 ms | 10.8x | Mixed approach: call OpenCV for this step |
| Erode 5x5 (grey) | medium (1920x1080) | 0.46 ms (oxipix) | 0.23 ms | 2.0x | Rust first; call OpenCV for this step if it is your bottleneck |
| Bilateral r=2 (RGB) | medium (1920x1080) | 9.38 ms (oxipix, threads) | 2.96 ms | 3.2x | Rust first; call OpenCV for this step if it is your bottleneck |
| Rotate 30 deg, scale 0.6 (RGB) | medium (1920x1080) | 1.50 ms (oxipix, threads) | 0.69 ms | 2.2x | Rust first; call OpenCV for this step if it is your bottleneck |
| pyrDown (RGB) | medium (1920x1080) | 0.45 ms (oxipix, threads) | 0.27 ms | 1.6x | Rust only |
| Gaussian blur (sigma=1.1, RGB) | big (3840x2160) | 4.62 ms (oxipix, threads) | 6.90 ms | 0.7x | Rust only |
| Sobel x (grey) | big (3840x2160) | 1.82 ms (oxipix) | 2.11 ms | 0.9x | Rust only |
| Canny 50/150 (grey) | big (3840x2160) | 16.59 ms (oxipix, threads) | 5.59 ms | 3.0x | Rust first; call OpenCV for this step if it is your bottleneck |
| Median 5x5 (RGB) | big (3840x2160) | 61.92 ms (oxipix, threads) | 5.48 ms | 11.3x | Mixed approach: call OpenCV for this step |
| Erode 5x5 (grey) | big (3840x2160) | 2.13 ms (oxipix) | 1.05 ms | 2.0x | Rust first; call OpenCV for this step if it is your bottleneck |
| Bilateral r=2 (RGB) | big (3840x2160) | 39.09 ms (oxipix, threads) | 12.09 ms | 3.2x | Rust first; call OpenCV for this step if it is your bottleneck |
| Rotate 30 deg, scale 0.6 (RGB) | big (3840x2160) | 6.36 ms (oxipix, threads) | 3.18 ms | 2.0x | Rust first; call OpenCV for this step if it is your bottleneck |
| pyrDown (RGB) | big (3840x2160) | 1.39 ms (oxipix, threads) | 1.25 ms | 1.1x | Rust only |

## What each option costs

| | Rust only | `oxipix` imgproc | `opencv` crate | Mixed |
|---|---|---|---|---|
| Native dependency | none | none | OpenCV 4.x or 5.x and clang at build time | OpenCV for the steps that use it |
| WebAssembly | yes | yes | no | not for the OpenCV steps |
| Threads | `imageproc` single-threaded | `parallel` feature, automatic size threshold | OpenCV's thread pool | both |
| Build time and binary size | small | small | large | large |
| Licence | MIT / Apache-2.0 | MIT / Apache-2.0 | crate MIT, OpenCV Apache-2.0 | both |

## Mixing the two

Keep the pipeline in Rust and call OpenCV only for the steps in the table that say so. Moving a picture between the worlds costs one copy each way (the guide's helpers in `snippets-opencv/src/lib.rs`; measured at under 30% of the call on the guide's photographs, and negligible next to a heavy step). In a hot loop, wrap the `image` buffer without copying with `Mat::new_rows_cols_with_data` for as long as the buffer lives.

