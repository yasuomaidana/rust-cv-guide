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

Show the real input and output pictures the example wrote, then the numbers it printed:
`![input](/img/examples/<example>/input.png)`. Numbers must be asserted by a test in `snippets/`, and the pictures are
checked against the committed PNGs by `figures_are_committed`.

## OpenCV to Rust

| OpenCV 5.1 | Rust |
|---|---|
| `cv::function` | `crate::function` |
