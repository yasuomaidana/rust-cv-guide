# rust-cv-guide

A guide for developers who know [OpenCV](https://docs.opencv.org/5.1.0/) and want to do the same work in pure Rust.
It maps OpenCV 5.1 modules and tutorials to Rust crates (oxipix, oximedia, image, imageproc and others), shows tested
code, and marks the gaps.

Site: <https://yasuomaidana.github.io/rust-cv-guide/> (deployed from `main`).

Not affiliated with OpenCV. The content is original; links go to docs.opencv.org.

## Layout

| Path | What |
|---|---|
| `website/` | Docusaurus site (private package, never published) |
| `.github/dendrite/directives.toml` | Quality policy enforced by CI and agents |

Planned: `catalog/` (OpenCV tree and coverage status), `tools/` (catalog checks), `snippets/` (compiled examples),
`comparisons/opencv/` (benchmarks).

## Develop

```bash
cd website
bun install
bun start      # preview on http://localhost:3000/rust-cv-guide/
bun run build  # fails on broken links
```

Work is tracked in Plane (`RUSTCVGUID-n`). One pull request per ticket, targeting `main` (or the trunk branch of its section ticket, which targets `main`); merging to `main` deploys the site.

## When is a tutorial "covered"?

The examples are the coverage tests. A tutorial may be marked `covered` or `partial` only when its example is tested, the page's numbers are asserted by that test, and the example runs live in the browser; `cargo run -p cvg-tools -- check` enforces it and CI runs it. The full list is in [`snippets/README.md`](snippets/README.md).

