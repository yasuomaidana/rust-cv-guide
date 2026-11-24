# cvg-tools

Run from the repository root.

- `cargo run -p cvg-tools -- check` fails when `catalog/coverage.yaml` and `catalog/opencv-5.1.yaml` disagree, or when
  coverage names a crate that is not in `catalog/crates.yaml`.
- `cargo run -p cvg-tools -- generate` rewrites `website/docs/opencv/` (one stub page per module and tutorial) and
  `website/docs/status.md` from the catalog and coverage files. Never edit those generated pages by hand.
