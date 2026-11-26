# Development plan

## Direction (decided 2026-11-24)

- The guide preserves OpenCV's **functionality**, not its API. Rust crates do not have to mirror OpenCV names or signatures.
- **oximedia** (`oximedia-cv`) gets 5 examples.
- Where both oximedia and `image`/`imageproc` implement a feature, the page shows **both solutions side by side**: code tabs and a live toggle on the same sliders. A test checks that the two agree within a stated tolerance.
- **oxipix** (0.1.0, now published) is also used, with default features only (no wgpu). Two examples (RUSTCVGUID-137): contours (`find_contours` + `approxPolyDP`) and homography (perspective unwarp), each beside the imageproc solution.
- **RustCV** is catalogued only (native camera/GUI facade, no wasm, no processing of its own).
- Every example has a live WebAssembly demo on its page.

## Definition of done (decided 2026-11-24, updated 2026-11-25)

The examples are the coverage tests. Every new tutorial or feature must:
1. Attempt **both** a pure-Rust (`image` / `imageproc`) solution and an **`oximedia-cv` / `oxipix`** solution side by side with shared input controls where both exist.
2. When `oximedia-cv` / `oxipix` lacks the capability, file a tracking ticket on `oxipix` (`dendrite chronos request oxipix --synapse cv-guide-oxipix`) and record `oxipix_ticket: OXI-<id>` on the entry in `catalog/coverage/*.yaml`.
3. Update `catalog/benchmarks.json` (`comparisons/opencv`) incrementally with OpenCV 5.1.0 vs pure-Rust vs `oximedia-cv`/`oxipix` release timings.
4. When a backend is significantly slower (`>= 3x` between Rust and `oximedia`/`oxipix`, or `>= 20x` vs OpenCV), record a root-cause `warning_note` in `catalog/benchmarks.json` (which `cvg-tools generate` renders automatically as a `:::warning Performance difference` admonition on the tutorial page) and file an upstream issue on `cool-japan/oximedia` (when inside `oximedia`) or on `oxipix` (when outside).

A tutorial is not finished until `cvg-tools check` proves all of the above: tested example, page numbers asserted by the test, live wasm export, tested alternatives, `oxipix_ticket` or `oximedia-cv`/`oxipix` crate, incremental `catalog/benchmarks.json` entry with `warning_note` for divergent rows, and no orphan examples (`RUSTCVGUID-138`, `RUSTCVGUID-152`, `RUSTCVGUID-5`).

## oximedia features

The five oximedia examples cover five different oximedia features, not five of one: bilateral filtering (done), Canny edges (done), ORB feature matching, Hough circles, PSNR/SSIM quality metrics.

## Tickets

| Ticket | Goal | Blocked by |
|---|---|---|
| RUSTCVGUID-133 | Group examples, tests and live bindings by section (`max_files_per_dir = 12`) | |
| RUSTCVGUID-134 | oximedia-cv alternatives: 5 examples beside image/imageproc | 133, matching imgproc pages |
| RUSTCVGUID-135 | imgproc umbrella: 10 more live examples | 133 |
| RUSTCVGUID-136 | Catalog RustCV, no example | |
| RUSTCVGUID-137 | oxipix examples: contours and homography beside imageproc | 133 |

## oximedia examples (134)

| # | Page | oximedia-cv | Counterpart |
|---|---|---|---|
| O1 | `tutorial_gausian_median_blur_bilateral_filter` | `BilateralFilter`, `GaussianBlur`, `MedianFilter` | `smoothing` (bilateral is oximedia-only) |
| O2 | `tutorial_canny_detector` | `CannyEdge` | `canny` |
| O3 | `tutorial_laplace_operator` | `LaplacianEdge` | imgproc example 2 |
| O4 | `tutorial_opening_closing_hats` | `MorphOperation`, `StructuringElement` | imgproc example 1 |
| O5 | `tutorial_hough_lines` | `hough_transform` | imgproc example 6 |

A spike comes first: `oximedia-cv` must build and run on `wasm32-unknown-unknown` (it depends on `rayon`). If it does not, its tab shows tested native code only and the reason goes in `catalog/crates.yaml`.

## imgproc examples (135)

opening_closing_hats, laplace_operator, filter_2d, pyramids, warp_affine, hough_lines, find_contours, distance_transform, template_matching, threshold_inRange. One PR per sub-ticket into the umbrella trunk; the trunk PR targets `main`.
