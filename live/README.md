# cvg-live

WebAssembly entry points for the guide's live examples. Each function runs the code of
`snippets/examples/<name>.rs` on RGBA pixels from a canvas and returns RGBA pixels.

Build for the site with `npm run wasm` in `website/` (needs the `wasm32-unknown-unknown` target and `wasm-bindgen-cli`).
