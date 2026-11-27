---
title: Threads and picture size
---

OpenCV runs on all the threads it finds by default, while the algorithm tables on the other benchmark pages pin it to one thread so that algorithms are compared, not core counts. This page puts the two side by side on small (640x480), medium (1920x1080) and big (3840x2160) pictures. The Rust columns are single-threaded; the last two columns say how far the best of them is from OpenCV on one thread and on all threads.

- **OpenCV threads by default:** 8
- **OpenCV found an OpenCL device:** yes

| Operation | Picture | OpenCV, 1 thread | OpenCV, all threads | imageproc | oxipix | Best Rust vs OpenCV 1 thread | Best Rust vs OpenCV all threads |
|---|---|---|---|---|---|---|---|
| Gaussian blur (sigma=1.1, RGB) | small (640x480) | 0.44 ms | 0.18 ms (2.4x faster) | 3.93 ms | 0.58 ms | 1.3x | 3.2x |
| Gaussian blur (sigma=1.1, RGB) | medium (1920x1080) | 2.96 ms | 0.86 ms (3.4x faster) | 26.49 ms | 3.96 ms | 1.3x | 4.6x |
| Gaussian blur (sigma=1.1, RGB) | big (3840x2160) | 11.58 ms | 3.02 ms (3.8x faster) | 106.35 ms | 16.53 ms | 1.4x | 5.5x |
| Sobel x (grey) | small (640x480) | 0.09 ms | 0.09 ms (1.0x faster) | 1.96 ms | 0.05 ms | 0.6x | 0.6x |
| Sobel x (grey) | medium (1920x1080) | 0.54 ms | 0.38 ms (1.4x faster) | 13.25 ms | 0.37 ms | 0.7x | 1.0x |
| Sobel x (grey) | big (3840x2160) | 2.24 ms | 1.72 ms (1.3x faster) | 53.20 ms | 2.04 ms | 0.9x | 1.2x |
| Canny 50/150 (grey) | small (640x480) | 1.38 ms | 0.47 ms (3.0x faster) | 10.39 ms | 0.99 ms | 0.7x | 2.1x |
| Canny 50/150 (grey) | medium (1920x1080) | 6.68 ms | 1.72 ms (3.9x faster) | 68.33 ms | 5.92 ms | 0.9x | 3.4x |
| Canny 50/150 (grey) | big (3840x2160) | 18.45 ms | 5.15 ms (3.6x faster) | 261.59 ms | 20.91 ms | 1.1x | 4.1x |
| Median 5x5 (RGB) | small (640x480) | 0.93 ms | 0.22 ms (4.2x faster) | 40.48 ms | 7.05 ms | 7.6x | 31.5x |
| Median 5x5 (RGB) | medium (1920x1080) | 5.71 ms | 1.43 ms (4.0x faster) | 293.41 ms | 47.86 ms | 8.4x | 33.4x |
| Median 5x5 (RGB) | big (3840x2160) | 21.79 ms | 4.48 ms (4.9x faster) | 1146.13 ms | 194.28 ms | 8.9x | 43.4x |
| Erode 5x5 (grey) | small (640x480) | 0.03 ms | 0.03 ms (1.0x faster) | 0.17 ms | 0.07 ms | 2.6x | 2.7x |
| Erode 5x5 (grey) | medium (1920x1080) | 0.17 ms | 0.17 ms (1.0x faster) | 0.96 ms | 0.45 ms | 2.7x | 2.6x |
| Erode 5x5 (grey) | big (3840x2160) | 0.66 ms | 0.66 ms (1.0x faster) | 3.79 ms | 2.05 ms | 3.1x | 3.1x |
| Bilateral r=2 (RGB) | small (640x480) | 1.84 ms | 0.68 ms (2.7x faster) | 18.60 ms | 6.23 ms | 3.4x | 9.2x |
| Bilateral r=2 (RGB) | medium (1920x1080) | 12.44 ms | 3.05 ms (4.1x faster) | 125.84 ms | 41.90 ms | 3.4x | 13.7x |
| Bilateral r=2 (RGB) | big (3840x2160) | 50.41 ms | 10.89 ms (4.6x faster) | 504.33 ms | 167.98 ms | 3.3x | 15.4x |
| Rotate 30 deg, scale 0.6 (RGB) | small (640x480) | 0.31 ms | 0.08 ms (3.9x faster) | 2.98 ms | 0.80 ms | 2.6x | 10.2x |
| Rotate 30 deg, scale 0.6 (RGB) | medium (1920x1080) | 2.01 ms | 0.47 ms (4.3x faster) | 19.97 ms | 5.42 ms | 2.7x | 11.6x |
| Rotate 30 deg, scale 0.6 (RGB) | big (3840x2160) | 8.67 ms | 2.00 ms (4.3x faster) | 81.10 ms | 22.72 ms | 2.6x | 11.4x |
| pyrDown (RGB) | small (640x480) | 0.07 ms | 0.03 ms (2.4x faster) | 3.13 ms | 0.20 ms | 2.9x | 7.0x |
| pyrDown (RGB) | medium (1920x1080) | 0.47 ms | 0.13 ms (3.7x faster) | 21.25 ms | 1.40 ms | 3.0x | 10.9x |
| pyrDown (RGB) | big (3840x2160) | 2.26 ms | 0.54 ms (4.2x faster) | 97.77 ms | 6.22 ms | 2.8x | 11.5x |
