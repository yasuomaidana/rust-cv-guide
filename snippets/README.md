# cvg-snippets

The compiled examples behind the guide pages. Each OpenCV tutorial that has a Rust answer gets:

- `examples/<name>.rs`: the code the page shows, kept short and runnable with `cargo run -p cvg-snippets --example <name>`;
- a library function the example calls, with tests that assert what the page prints.

Name the example in `catalog/coverage/<section>.yaml` (`snippet: <name>`) and put `{{snippet}}` in
`website/guide/<section>/<id>.md`; `cargo run -p cvg-tools -- generate` copies the source into the page.

Inputs are generated in code with a fixed seed (see `common::gradient`), so results are reproducible. Dependencies must be
pure Rust.
