# OpenCV 5.1.0 vs Pure Rust vs oximedia/oxipix Benchmark Harness

Standalone runner (excluded from the root workspace so CI does not require native OpenCV) that executes:

1. `bench.cpp` compiled with `-O3` against Homebrew OpenCV 5.1.0 (`opencv5`).
2. `src/main.rs` compiled with `--release` calling the exact functions from `snippets/examples/`.

It writes measured median wall-clock timings (excluding image decode and fixture setup) to `catalog/benchmarks.json`.

## Running

```bash
PKG_CONFIG_PATH=/opt/homebrew/opt/opencv/lib/pkgconfig bash -c '
  clang++ -O3 -std=c++17 bench.cpp $(pkg-config --cflags --libs opencv5) -o bench_cpp
'
./bench_cpp > /tmp/opencv_bench.json
cargo run --release -- /tmp/opencv_bench.json ../../catalog/benchmarks.json
```
