---
title: Performance benchmarks
---

Incremental performance reference comparing native **OpenCV**, pure Rust (`image` + `imageproc`), and **`oximedia-cv` / `oxipix`** across every implemented tutorial.

- **Host / profile:** macos aarch64 (`cargo run --release`)
- **OpenCV:** 5.1.0
- **Crates:** `oximedia-cv` 0.2.1, `oxipix` 0.1.0, `imageproc` 0.27.0
- **Divergence thresholds:** >= 3.0x (`oximedia` vs pure Rust) or >= 20.0x vs OpenCV triggers an automated warning callout.

| Tutorial | Operation | Input | OpenCV | Pure Rust | oximedia / oxipix | oxi / Rust | Notes |
|---|---|---|---|---|---|---|---|
| [Adding (blending) two images using OpenCV](opencv/core/tutorial_adding_images.mdx) | addWeighted (0.5 / 0.5) | 320x240 RGB | 0.066 ms | 0.314 ms | 0.558 ms | 1.78x | — |
| [Changing the contrast and brightness of an image!](opencv/core/tutorial_basic_linear_transform.mdx) | Contrast + brightness (a=1.3, b=40) | 600x400 RGB | 0.110 ms | 0.463 ms | — (OXI-88) | — | — |
| [Video Input with OpenCV and similarity measurement](opencv/app/tutorial_video_input_psnr_ssim.mdx) | PSNR | 640x480 grey | 0.256 ms | 0.396 ms | 0.400 ms | 1.01x | — |
| [Video Input with OpenCV and similarity measurement](opencv/app/tutorial_video_input_psnr_ssim.mdx) | SSIM (11x11 Gaussian) | 640x480 grey | 6.360 ms | 7.128 ms | 105.017 ms | 14.73x | oximedia-cv's SSIM (14.7x slower than pure Rust separable SSIM, 16.5x slower than OpenCV) runs a non-separable 2D 11x11 window loop with per-tap bounds checks per pixel instead of 5 separable 1D passes (cool-japan/oximedia#31). |
| [Feature Detection](opencv/features/tutorial_feature_detection.mdx) | FAST-9 keypoints (t=30, max=250) | 640x480 grey | 1.149 ms | 2.370 ms | 6.971 ms | 2.94x | — |
| [Feature Detection](opencv/features/tutorial_feature_detection.mdx) | ORB keypoints (t=30, max=250) | 640x480 grey | 13.926 ms | 2.323 ms | 3.842 ms | 1.65x | — |
| [Basic concepts of the homography explained with code](opencv/features/tutorial_homography.mdx) | Perspective unwarp (4-point quad) | 640x480 RGB | 1.020 ms | 4.178 ms | 6.722 ms | 1.61x | — |
| [Basic Thresholding Operations](opencv/imgproc/tutorial_threshold.mdx) | Otsu binary threshold | 384x303 grey | 0.072 ms | 0.045 ms | 0.163 ms | 3.62x | The oxipix threshold helper runs a per-pixel GrayImage::from_fn wrapper over otsu_threshold (~3.6x slower than imageproc's tight slice loop; native slice thresholding tracked in OXI-89). |
| [Thresholding Operations using inRange](opencv/imgproc/tutorial_threshold_inRange.mdx) | HSV red range mask | 320x240 RGB | 0.269 ms | 0.193 ms | 0.449 ms | 2.33x | — |
| [Histogram Equalization](opencv/imgproc/tutorial_histogram_equalization.mdx) | Histogram equalisation | 640x480 grey | 0.442 ms | 0.356 ms | 0.311 ms | 0.87x | — |
| [Eroding and Dilating](opencv/imgproc/tutorial_erosion_dilatation.mdx) | Erode (5x5 square, r=2) | 384x303 grey | 0.018 ms | 0.278 ms | 3.279 ms | 11.79x | oximedia-cv erode is 11.8x slower than imageproc (182x slower than OpenCV) because StructuringElement::iter_active recomputes (i % w, i / w) offsets and checks bounds per tap with no separable rectangular fast path (cool-japan/oximedia#30). |
| [More Morphology Transformations](opencv/imgproc/tutorial_opening_closing_hats.mdx) | Top-hat morphology (7x7, r=3) | 384x303 grey | 0.068 ms | 0.420 ms | 14.045 ms | 33.44x | oximedia-cv top_hat is 33.4x slower than imageproc (206.5x slower than OpenCV) for a 7x7 square window due to non-separable 2D erode/dilate with per-pixel iter_active() coordinate division and bounds checks (cool-japan/oximedia#30). |
| [Image Segmentation with Distance Transform and Watershed Algorithm](opencv/imgproc/tutorial_distance_transform.mdx) | Distance transform (L2) | 384x303 mask | 0.503 ms | 0.826 ms | — (OXI-91) | — | — |
| [Finding contours in your image](opencv/imgproc/tutorial_find_contours.mdx) | Find contours + approxPolyDP | 560x420 mask | 0.374 ms | 0.588 ms | 0.286 ms | 0.49x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Box blur (5x5, r=2) | 451x300 RGB | 0.417 ms | 1.351 ms | 1.576 ms | 1.17x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Gaussian blur (sigma=1.1) | 451x300 RGB | 0.345 ms | 1.797 ms | 4.842 ms | 2.69x | — |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Median filter (5x5, r=2) | 451x300 RGB | 0.873 ms | 17.887 ms | 38.468 ms | 2.15x | Median 5x5 is 20.5x (imageproc) to 44.1x (oximedia-cv) slower than OpenCV: oximedia-cv heap-allocates a Vec and calls sort_unstable() at every pixel instead of using a 256-bin sliding histogram (cool-japan/oximedia#29, OXI-93). |
| [Smoothing Images](opencv/imgproc/tutorial_gausian_median_blur_bilateral_filter.mdx) | Bilateral filter (d=5, r=2, sc=30) | 451x300 RGB | 1.618 ms | 9.245 ms | 33.128 ms | 3.58x | oximedia-cv bilateral filter is 3.6x slower than imageproc (20.5x slower than OpenCV): it runs 3 grayscale passes in f64 calling f64::exp() inside the 4-deep loop without a 256-entry range LUT (cool-japan/oximedia#29, OXI-93). |
| [Making your own linear filters!](opencv/imgproc/tutorial_filter_2d.mdx) | 2D convolution (5x5 box kernel) | 451x300 RGB | 1.329 ms | 3.526 ms | 8.370 ms | 2.37x | — |
| [Laplace Operator](opencv/imgproc/tutorial_laplace_operator.mdx) | Gaussian blur + Laplacian edge | 640x480 grey | 1.620 ms | 3.473 ms | 5.703 ms | 1.64x | — |
| [Sobel Derivatives](opencv/imgproc/tutorial_sobel_derivatives.mdx) | Sobel X derivative | 640x480 grey | 0.335 ms | 3.863 ms | 0.788 ms | 0.20x | Pure Rust imageproc Sobel X is ~4.9x slower than oximedia-cv (11.5x slower than OpenCV) because horizontal_sobel allocates an intermediate i16 buffer and remaps pixels separately, while oximedia-cv writes directly on the flat slice. |
| [Canny Edge Detector](opencv/imgproc/tutorial_canny_detector.mdx) | Gaussian blur + Canny (50/150) | 640x480 grey | 4.391 ms | 19.769 ms | 22.257 ms | 1.13x | — |
| [Hough Line Transform](opencv/imgproc/tutorial_hough_lines.mdx) | Canny + standard Hough lines | 640x480 grey | 21.470 ms | 21.845 ms | 42.239 ms | 1.93x | — |
| [Hough Circle Transform](opencv/imgproc/tutorial_hough_circle.mdx) | Canny + circle Hough (r=15..40) | 384x303 grey | 6.885 ms | — | 63.094 ms | — | — |
| [Image Pyramids](opencv/imgproc/tutorial_pyramids.mdx) | 3-level pyrDown | 451x300 RGB | 0.107 ms | 1.833 ms | 0.888 ms | 0.48x | — |
| [Affine Transformations](opencv/imgproc/tutorial_warp_affine.mdx) | Rotate 30 deg + scale 0.6 | 512x512 RGB | 0.626 ms | 2.745 ms | 1.843 ms | 0.67x | — |
| [Template Matching](opencv/imgproc/tutorial_template_matching.mdx) | Template match (TM_SQDIFF) | 451x300 vs 40x40 grey | 2.765 ms | 120.204 ms | — (OXI-92) | — | imageproc template matching is 43.5x slower than OpenCV because it evaluates naive spatial O(W*H*w*h) sliding sums instead of FFT cross-correlation (oxipix support tracked in OXI-92). |
