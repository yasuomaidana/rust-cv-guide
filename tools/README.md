# cvg-tools

Run from the repository root.

- `cargo run -p cvg-tools -- check` fails when the catalog (`catalog/opencv-5.1.yaml`), the per-section coverage files
  (`catalog/coverage/*.yaml`) and `catalog/crates.yaml` disagree. It also enforces evidence:
  - `covered` / `partial` needs crates, a `snippet` that exists as `snippets/examples/<snippet>.rs`, and a guide body at
    `website/guide/<section>/<id>.md`;
  - `gap` / `not-planned` needs a `reason`;
  - a guide body that matches no tutorial is an error, and so is a key that appears in two coverage files.
- `cargo run -p cvg-tools -- generate` rewrites `website/docs/opencv/` and `website/docs/status.md`. Each page is a
  generated header (status, reason, crates, OpenCV source link) followed by the hand-written guide body, with
  `{{snippet}}` replaced by the example's source. Never edit the generated pages by hand.
