#!/usr/bin/env bash
# Builds the native OpenCV benchmark, runs the default and the optimised Rust builds, and merges them
# into catalog/benchmarks.json. Usage: ./run.sh [output.json]
set -euo pipefail
cd "$(dirname "$0")"
out="${1:-../../catalog/benchmarks.json}"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
export PKG_CONFIG_PATH="${PKG_CONFIG_PATH:-/opt/homebrew/opt/opencv/lib/pkgconfig}"
# The `opencv` crate needs clang to generate its bindings and the OpenCV libraries at run time.
export LIBCLANG_PATH="${LIBCLANG_PATH:-/opt/homebrew/opt/llvm/lib}"
export DYLD_FALLBACK_LIBRARY_PATH="${DYLD_FALLBACK_LIBRARY_PATH:-/opt/homebrew/opt/opencv/lib:/opt/homebrew/opt/llvm/lib}"

clang++ -O3 -std=c++17 bench.cpp $(pkg-config --cflags --libs opencv5) -o "$work/bench_cpp"
"$work/bench_cpp" ../../snippets/assets > "$work/opencv.json"

# Optimised build first: fat LTO, one codegen unit, host CPU.
CARGO_PROFILE_RELEASE_LTO=fat CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 CARGO_PROFILE_RELEASE_PANIC=abort \
RUSTFLAGS="-C target-cpu=native" \
  cargo run --release -q --target-dir "$work/target-opt" -- "$work/opencv.json" "$work/opt.json"

cargo run --release -q -- "$work/opencv.json" "$out" "$work/opt.json"
