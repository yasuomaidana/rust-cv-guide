//! Keeps the OpenCV 5.1 catalog, the coverage file and the generated pages in step.

pub mod benchmarks;
pub mod check;
pub mod generate;
pub mod load;
pub mod model;
pub mod perf_queue;
pub mod proof;
pub mod roadmap;

use benchmarks::Benchmarks;
use load::at;
use model::{Catalog, Coverage, Crates, Sources};
use std::fs;
use std::path::Path;

/// Runs `check` or `generate` against the repository at `root`; the message is what to print.
pub fn run(args: &[String], root: &Path) -> Result<String, String> {
    match args.first().map(String::as_str) {
        Some("check") => {
            checked(root).map(|_| "catalog, coverage and crates are consistent".into())
        }
        Some("generate") => generate(root),
        Some("perf-queue") => perf_queue_files(root),
        _ => Err("usage: cvg-tools <check|generate|perf-queue>".into()),
    }
}

struct Inputs {
    catalog: Catalog,
    coverage: Coverage,
    sources: Sources,
    benchmarks: Benchmarks,
    queue: perf_queue::QueueMeta,
}

/// Writes the queue files from the benchmarks without enforcing issue links; says which items lack one.
fn perf_queue_files(root: &Path) -> Result<String, String> {
    let benchmarks = load::benchmarks(root)?;
    let meta = load::queue_meta(root)?;
    let queue = perf_queue::build(&benchmarks, &meta);
    for (path, content) in perf_queue::outputs(&queue) {
        let path = root.join(path);
        fs::create_dir_all(path.parent().unwrap_or(root)).map_err(at(&path))?;
        fs::write(&path, content).map_err(at(&path))?;
    }
    let missing = perf_queue::problems(&benchmarks, &meta);
    Ok(format!(
        "queue of {} items written; {} problems still to fix\n{}",
        queue.queue.len(),
        missing.len(),
        missing.join("\n")
    ))
}

/// `verified`, plus a check that the committed queue files are what `generate` would write.
fn checked(root: &Path) -> Result<Inputs, String> {
    let inputs = verified(root)?;
    let stale: Vec<String> = queue_outputs(&inputs)
        .into_iter()
        .filter(|(path, content)| {
            fs::read_to_string(root.join(path)).ok().as_ref() != Some(content)
        })
        .map(|(path, _)| format!("{path} is out of date: run `cargo run -p cvg-tools -- generate`"))
        .collect();
    if stale.is_empty() {
        Ok(inputs)
    } else {
        Err(stale.join("\n"))
    }
}

/// The queue files for the loaded benchmarks; none while there are no benchmarks.
fn queue_outputs(inputs: &Inputs) -> Vec<(String, String)> {
    if inputs.benchmarks.benchmarks.is_empty() {
        return Vec::new();
    }
    perf_queue::outputs(&perf_queue::build(&inputs.benchmarks, &inputs.queue))
}

fn verified(root: &Path) -> Result<Inputs, String> {
    let catalog: Catalog = load::yaml(&root.join("catalog/opencv-5.1.yaml"))?;
    let crates: Crates = load::yaml(&root.join("catalog/crates.yaml"))?;
    let coverage = load::coverage(root)?;
    let sources = load::sources(root)?;
    let benchmarks = load::benchmarks(root)?;
    let queue = load::queue_meta(root)?;
    let mut problems = check::problems(&catalog, &coverage, &crates, &sources, &benchmarks);
    problems.extend(perf_queue::problems(&benchmarks, &queue));
    if problems.is_empty() {
        Ok(Inputs {
            catalog,
            coverage,
            sources,
            benchmarks,
            queue,
        })
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
    let pages = generate::pages(
        &inputs.catalog,
        &inputs.coverage,
        &inputs.sources,
        &inputs.benchmarks,
    );
    for page in &pages {
        let path = site.join(&page.path);
        fs::create_dir_all(path.parent().unwrap_or(&site)).map_err(at(&path))?;
        fs::write(&path, &page.content).map_err(at(&path))?;
    }
    let extra = queue_outputs(&inputs);
    for (path, content) in &extra {
        let path = root.join(path);
        fs::create_dir_all(path.parent().unwrap_or(root)).map_err(at(&path))?;
        fs::write(&path, content).map_err(at(&path))?;
    }
    Ok(format!(
        "wrote {} pages and {} queue files",
        pages.len(),
        extra.len()
    ))
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
        fs::create_dir_all(dir.path().join("catalog/coverage")).unwrap();
        for (name, text) in [
            ("opencv-5.1.yaml", CATALOG),
            ("coverage/all.yaml", coverage),
            ("crates.yaml", CRATES),
        ] {
            fs::write(dir.path().join("catalog").join(name), text).unwrap();
        }
        dir
    }

    const OK_COVERAGE: &str = "modules:\n  core: {status: covered, crates: [ndarray]}\ntutorials:\n  tutorial_mat: {status: gap, reason: nothing yet}\n";

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
    #[case("modules: [", "all.yaml")]
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
        assert_eq!(
            call("generate", dir.path()).unwrap(),
            "wrote 2 pages and 0 queue files"
        );
        assert!(!stale.exists());
        assert!(dir.path().join("website/docs/roadmap.md").exists());
        assert!(dir.path().join("website/docs/benchmarks.md").exists());
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

    #[rstest]
    fn generate_embeds_the_guide_body_and_snippet(repo: TempDir) {
        let covered = "tutorials:\n  tutorial_mat: {status: covered, crates: [ndarray], oxipix_ticket: OXI-1, snippet: core/blend}\nmodules:\n  core: {status: gap, reason: r}\n";
        fs::write(repo.path().join("catalog/coverage/all.yaml"), covered).unwrap();
        let write = |relative: &str, text: &str| {
            let path = repo.path().join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        };
        write(
            "catalog/benchmarks.json",
            r#"{"benchmarks":[{"id":"mat","tutorial":"tutorial_mat","section":"core","operation":"op","input":"4x4"}]}"#,
        );
        write(
            "website/guide/core/tutorial_mat.md",
            "Intro.\n\n{{snippet}}\n\n<LiveExample fn=\"blend\" />\n",
        );
        write("snippets/examples/core/blend.rs", "fn main() {}\n");
        write("snippets/tests/core/blend.rs", "fn t() {}\n");
        write(
            "live/src/lib.rs",
            "#[path = \"../../snippets/examples/core/blend.rs\"]\nmod b;\npub fn blend() {}\n",
        );
        call("generate", repo.path()).unwrap();
        let page = fs::read_to_string(
            repo.path()
                .join("website/docs/opencv/core/tutorial_mat.mdx"),
        )
        .unwrap();
        assert!(
            page.contains("Intro.") && page.contains("```rust\nfn main() {}\n```"),
            "{page}"
        );
    }
}
