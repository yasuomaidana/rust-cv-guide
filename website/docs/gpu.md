---
title: Does the GPU help?
---

Short answer: not enough to chase. On the reference machine the multi-threaded CPU kernels are as fast as or faster than the GPU for the operations that exist on both, and OpenCV on all threads is faster still.

The `oxipix` crate has a `gpu` feature (wgpu compute shaders) with a 3x3 filter and a perspective warp. The `gpu_vs_threads` example in the `oxipix` repository times them, **including upload and readback**, against the `imgproc` kernels of the same crate on one thread and on all threads, on RGBA pictures of three sizes (`cargo run --release --example gpu_vs_threads --features gpu,parallel`). Two runs on an Apple M-series chip with 8 cores, in milliseconds:

| Size | GPU 3x3 filter | CPU, 1 thread | CPU, all threads | GPU warp | CPU, 1 thread | CPU, all threads |
|---|---|---|---|---|---|---|
| small, 640x480 | 1.1 | 1.0 | 0.32 | 0.9 | 2.9 | 0.6 to 0.8 |
| medium, 1920x1080 | 3.3 to 5.6 | 6.9 to 7.2 | 1.9 to 2.0 | 2.9 to 4.6 | 19.9 | 4.1 to 5.3 |
| big, 3840x2160 | 13.0 to 16.7 | 28 to 29 | 7.6 to 8.0 | 13.3 to 13.7 | 80.5 | 17.5 |

What it shows:

- Against **one** CPU thread the GPU wins from the medium size up (2x on the filter, 4x to 6x on the warp), and loses on small pictures where the fixed cost of a dispatch dominates.
- Against **all** CPU threads the picture changes: the threaded filter beats the GPU at every size, and the GPU warp is ahead by only 1.1x to 1.4x at medium and big sizes.
- OpenCV on all threads does the same warp in about 0.5 ms (medium) and 1.9 ms (big), several times faster than the GPU path, and its OpenCL path is not used by the guide's `Mat` examples. See [Threads and picture size](threads-and-size.md).

Apple silicon shares memory between the CPU and the GPU, which makes the transfers cheap; on a machine with a discrete GPU the transfers cost more and the GPU wins less. A GPU is worth considering when many operations are chained on the device without reading back in between, which these single-call numbers do not measure.

Because of this the guide does not port more kernels to the GPU. The decision rule was to port a kernel only if it beats the best CPU backend on all threads at two megapixels or more, transfers included, and none of the measured ones does clearly enough.
