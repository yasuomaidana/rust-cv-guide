# Changelog

## Unreleased

- Shared helpers: `common::gradient`, `common::fnv1a`, `common::sample`, `common::describe`.
- Examples expose parameterised functions that the `live/` crate runs in the browser; output pictures are no longer committed.
- [RUSTCVGUID-184] `brief` and `oxi_features` modules: shared BRIEF sampling, FAST non-max suppression and the oximedia FAST + BRIEF pipeline; feature examples run one pipeline on both crates; imageproc canny snippet no longer blurs twice; Hough snippets take a suppression radius.
