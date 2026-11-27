# snippets-opencv

The guide's tutorials with the [`opencv`](https://crates.io/crates/opencv) crate, which calls a native OpenCV. They are the
third option beside the pure-Rust (`image`, `imageproc`) and the `oxipix` / `oximedia-cv` versions, and appear as an
`opencv` tab on each tutorial page.

The examples themselves live in `../snippets/examples/opencv_*` so the pages can show them; this crate compiles them
(`[[example]]` entries) and `tests/parity.rs` compares each one with its pure-Rust twin on the guide's photographs.

It is a standalone crate, not part of the root workspace, because it needs a native OpenCV 5.x and `clang` at build time,
which CI does not have and WebAssembly cannot use.

```bash
export PKG_CONFIG_PATH=/opt/homebrew/opt/opencv/lib/pkgconfig LIBCLANG_PATH=/opt/homebrew/opt/llvm/lib
export DYLD_FALLBACK_LIBRARY_PATH=/opt/homebrew/opt/opencv/lib:/opt/homebrew/opt/llvm/lib   # macOS runtime
cargo test                      # parity with the pure-Rust versions
cargo run --example canny
```

The `image` <-> `Mat` conversions in `src/lib.rs` copy; wrap the buffer without copying
(`Mat::new_rows_cols_with_data`) in a real pipeline. The benchmark page says what the copies cost.
