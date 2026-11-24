//! Runs the real binary against this repository's catalog.

use rstest::rstest;
use std::path::Path;
use std::process::Command;

#[rstest]
#[case("check", true, "consistent")]
#[case("bogus", false, "usage")]
fn binary_reports_the_outcome(#[case] arg: &str, #[case] ok: bool, #[case] text: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let out = Command::new(env!("CARGO_BIN_EXE_cvg-tools"))
        .arg(arg)
        .current_dir(root)
        .output()
        .unwrap();
    let shown = String::from_utf8_lossy(if ok { &out.stdout } else { &out.stderr }).into_owned();
    assert_eq!(out.status.success(), ok, "{shown}");
    assert!(shown.contains(text), "{shown}");
}
