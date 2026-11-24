//! Keeps the OpenCV 5.1 catalog, the coverage file and the generated pages in step.

pub mod check;
pub mod generate;
pub mod model;

use model::{Catalog, Coverage, Crates};
use serde::de::DeserializeOwned;
use std::fmt::Display;
use std::fs;
use std::path::Path;

/// Runs `check` or `generate` against the repository at `root`; the message is what to print.
pub fn run(args: &[String], root: &Path) -> Result<String, String> {
    match args.first().map(String::as_str) {
        Some("check") => {
            verified(root).map(|_| "catalog, coverage and crates are consistent".into())
        }
        Some("generate") => generate(root),
        _ => Err("usage: cvg-tools <check|generate>".into()),
    }
}

fn at<E: Display>(path: &Path) -> impl Fn(E) -> String + '_ {
    move |e| format!("{}: {e}", path.display())
}

fn load<T: DeserializeOwned>(root: &Path, name: &str) -> Result<T, String> {
    let path = root.join("catalog").join(name);
    let text = fs::read_to_string(&path).map_err(at(&path))?;
    serde_yaml_ng::from_str(&text).map_err(at(&path))
}

struct Inputs {
    catalog: Catalog,
    coverage: Coverage,
}

fn verified(root: &Path) -> Result<Inputs, String> {
    let catalog: Catalog = load(root, "opencv-5.1.yaml")?;
    let coverage: Coverage = load(root, "coverage.yaml")?;
    let crates: Crates = load(root, "crates.yaml")?;
    let problems = check::problems(&catalog, &coverage, &crates);
    if problems.is_empty() {
        Ok(Inputs { catalog, coverage })
    } else {
        Err(problems.join("\n"))
    }
}

fn generate(root: &Path) -> Result<String, String> {
    let inputs = verified(root)?;
    let site = root.join("website");
    let stale = site.join("docs/opencv");
    if stale.exists() {
        fs::remove_dir_all(&stale).map_err(at(&stale))?;
    }
    let pages = generate::pages(&inputs.catalog, &inputs.coverage);
    for page in &pages {
        let path = site.join(&page.path);
        fs::create_dir_all(path.parent().unwrap_or(&site)).map_err(at(&path))?;
        fs::write(&path, &page.content).map_err(at(&path))?;
    }
    Ok(format!("wrote {} pages", pages.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};
    use tempfile::TempDir;

    const CATALOG: &str = "modules:\n  - {name: core, title: Core, source: modules/core}\ntutorials:\n  - {id: tutorial_mat, section: core, title: Mat, source: doc/mat.markdown}\n";
    const CRATES: &str = "crates:\n  - {name: ndarray}\n";

    fn repo_with(coverage: &str) -> TempDir {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("catalog")).unwrap();
        for (name, text) in [
            ("opencv-5.1.yaml", CATALOG),
            ("coverage.yaml", coverage),
            ("crates.yaml", CRATES),
        ] {
            fs::write(dir.path().join("catalog").join(name), text).unwrap();
        }
        dir
    }

    const OK_COVERAGE: &str = "modules:\n  core: {status: covered, crates: [ndarray]}\ntutorials:\n  tutorial_mat: {status: gap}\n";

    #[fixture]
    fn repo() -> TempDir {
        repo_with(OK_COVERAGE)
    }

    fn call(arg: &str, root: &Path) -> Result<String, String> {
        run(&[arg.to_string()], root)
    }

    #[rstest]
    fn check_accepts_consistent_files(repo: TempDir) {
        assert!(call("check", repo.path()).unwrap().contains("consistent"));
    }

    #[rstest]
    #[case("modules: {}\ntutorials: {}\n", "module core has no coverage entry")]
    #[case("modules: [", "coverage.yaml")]
    fn check_reports_bad_coverage(#[case] coverage: &str, #[case] expected: &str) {
        let err = call("check", repo_with(coverage).path()).unwrap_err();
        assert!(err.contains(expected), "{err}");
    }

    #[rstest]
    fn check_reports_a_missing_file(repo: TempDir) {
        fs::remove_file(repo.path().join("catalog/crates.yaml")).unwrap();
        assert!(
            call("check", repo.path())
                .unwrap_err()
                .contains("crates.yaml")
        );
    }

    #[test]
    fn unknown_command_prints_usage() {
        assert!(run(&[], Path::new(".")).unwrap_err().contains("usage"));
    }

    #[rstest]
    fn generate_replaces_stale_pages(repo: TempDir) {
        let dir = repo;
        let stale = dir.path().join("website/docs/opencv/old/gone.md");
        fs::create_dir_all(stale.parent().unwrap()).unwrap();
        fs::write(&stale, "x").unwrap();
        assert_eq!(call("generate", dir.path()).unwrap(), "wrote 5 pages");
        assert!(!stale.exists());
        assert!(dir.path().join("website/docs/status.md").exists());
    }

    #[rstest]
    fn generate_reports_unwritable_output(repo: TempDir) {
        fs::write(repo.path().join("website"), "a file, not a directory").unwrap();
        assert!(
            call("generate", repo.path())
                .unwrap_err()
                .contains("website")
        );
    }
}
