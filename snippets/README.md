# cvg-snippets

The compiled examples behind the guide pages. Each OpenCV tutorial that has a Rust answer gets:

- `examples/<name>.rs`: the code the page shows, kept short and runnable with `cargo run -p cvg-snippets --example <name>`;
- a library function the example calls, with tests that assert what the page prints.

Name the example in `catalog/coverage/<section>.yaml` (`snippet: <name>`) and put `{{snippet}}` in
`website/guide/<section>/<id>.md`; `cargo run -p cvg-tools -- generate` copies the source into the page.

Inputs are real photographs from `assets/` (`common::sample("name.jpg")`); every file there is listed with its source and
licence in `assets/ATTRIBUTION.md`, and a photograph without a clear licence is not added. Unit tests may still use the
synthetic `common::gradient`.

Each example exposes:

- `figures() -> Vec<Figure>`: the input and output pictures, named `input`, `output`, ...;
- `report() -> String`: the numbers the page prints;
- `main()`: `common::save_figures("<example>", &figures())` writes them to `website/static/img/examples/<example>/` and prints the report.

Its test asserts `report()`, and one test asserts `common::figures_are_committed(..)` before calling `main()`, so the pictures
on the page cannot drift from the code. Regenerate them with `cargo run -p cvg-snippets --example <name>`.

Dependencies must be pure Rust.
