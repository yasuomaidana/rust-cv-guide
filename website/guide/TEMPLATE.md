<!-- Copy to website/guide/<section>/<id>.md. The generated header (title, status, crates, OpenCV source link) is added
     above this text, so start at "Goal". Write in your own words; do not copy OpenCV's text. -->

## Goal

What the reader will learn, in one or two sentences.

## Theory

The idea behind it, with the maths if it helps. Cite papers or the OpenCV page rather than reproducing them.

## Code

{{snippet}}

## Explanation

Walk through the code above step by step.

## Result

Put a live example first, then the numbers the code prints:
`<LiveExample fn="edges" sample="building.jpg" params={[{name: "low", label: "Low", min: 0, max: 255, step: 1, value: 50}]} />`
(`fn` is an export of a module in `live/src/`; `sample` is a file of `snippets/assets/`; `options: [..]` makes a drop-down).
Numbers must be asserted by a test in `snippets/`.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::function` | `crate::function` |

A page is only valid when `cargo run -p cvg-tools -- check` passes: its first ```text block must be asserted verbatim by the
example's test, its `<LiveExample fn="..">` must name a `#[wasm_bindgen]` export of `live/`, and every alternative crate needs its own tested example.

