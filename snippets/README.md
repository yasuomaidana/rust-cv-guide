# cvg-snippets

The compiled examples behind the guide pages. Each OpenCV tutorial that has a Rust answer gets:

- `examples/<group>/<name>.rs`: the code the page shows, kept short and runnable with `cargo run -p cvg-snippets --example <name>`. Groups keep each folder under the 12-file limit (`core`, `imgproc_filters`, `imgproc_morph`, `imgproc_edges`, `imgproc_color`, ...); a new file needs `[[example]]` and `[[test]]` entries in `Cargo.toml`;
- a library function the example calls, with tests that assert what the page prints.

Name the example in `catalog/coverage/<section>.yaml` (`snippet: <group>/<name>`) and put `{{snippet}}` in
`website/guide/<section>/<id>.md`; `cargo run -p cvg-tools -- generate` copies the source into the page.

Inputs are real photographs from `assets/` (`common::sample("name.jpg")`); every file there is listed with its source and
licence in `assets/ATTRIBUTION.md`, and a photograph without a clear licence is not added. Unit tests may still use the
synthetic `common::gradient`.

Each example exposes:

- one or more small pure functions with the parameters a reader would change (`detect_edges(&gray, low, high)`);
- `figures() -> Vec<Figure>`: the pictures for the page's default settings, which `report()` describes with `common::describe`;
- `report() -> String`: the numbers the page prints, asserted by its test;
- `main()`: prints the report.

The pictures on the site are not committed: the `live/` crate (`cvg-live`) compiles the same files to WebAssembly and the page's
`<LiveExample>` runs them in the browser on the sample photo or the reader's own image. To make a new example live, add a
`#[wasm_bindgen]` function for it in the matching module of `live/src/` (`basics`, `filters`, `morph`, `edges`, `color`) and a `<LiveExample fn=".." sample=".." params={[..]} />` to its guide page.

Dependencies must be pure Rust.
