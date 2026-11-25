//! Reads the catalog, the per-section coverage files and the hand-written sources from disk.

use crate::model::{Coverage, Sources};
use serde::de::DeserializeOwned;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

pub fn at<E: Display>(path: &Path) -> impl Fn(E) -> String + '_ {
    move |e| format!("{}: {e}", path.display())
}

/// Parses one YAML file under `catalog/`.
pub fn yaml<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = fs::read_to_string(path).map_err(at(path))?;
    serde_yaml_ng::from_str(&text).map_err(at(path))
}

/// Files in `dir` with the given extension, sorted; a missing directory has none.
fn files(dir: &Path, extension: &str) -> Result<Vec<PathBuf>, String> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let entries = fs::read_dir(dir).map_err(at(dir))?;
    let mut found: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == extension))
        .collect();
    found.sort();
    Ok(found)
}

fn stem(path: &Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

/// Merges every `catalog/coverage/*.yaml`; a key in two files is an error.
pub fn coverage(root: &Path) -> Result<Coverage, String> {
    let mut merged = Coverage::default();
    for path in files(&root.join("catalog/coverage"), "yaml")? {
        let part: Coverage = yaml(&path)?;
        for (id, entry) in part.modules {
            if merged.modules.insert(id.clone(), entry).is_some() {
                return Err(format!(
                    "{}: module {id} is already covered in another file",
                    path.display()
                ));
            }
        }
        for (id, entry) in part.tutorials {
            if merged.tutorials.insert(id.clone(), entry).is_some() {
                return Err(format!(
                    "{}: tutorial {id} is already covered in another file",
                    path.display()
                ));
            }
        }
    }
    Ok(merged)
}

fn read_all(
    paths: Vec<PathBuf>,
    key: impl Fn(&Path) -> String,
) -> Result<BTreeMap<String, String>, String> {
    let mut out = BTreeMap::new();
    for path in paths {
        out.insert(key(&path), fs::read_to_string(&path).map_err(at(&path))?);
    }
    Ok(out)
}

/// Reads `<dir>/<group>/*.<extension>` for every group folder, keyed `<group>/<file stem>`.
fn grouped(dir: &Path, extension: &str) -> Result<BTreeMap<String, String>, String> {
    let mut found = BTreeMap::new();
    let groups = fs::read_dir(dir).map(|d| d.filter_map(|e| e.ok()).collect::<Vec<_>>());
    for group in groups.unwrap_or_default() {
        let name = group.file_name().to_string_lossy().into_owned();
        let key = |p: &Path| format!("{name}/{}", stem(p));
        found.extend(read_all(files(&group.path(), extension)?, key)?);
    }
    Ok(found)
}

/// Guide bodies from `website/guide/<section>/*.md` and examples from `snippets/examples/<group>/*.rs`.
pub fn sources(root: &Path) -> Result<Sources, String> {
    let bodies = grouped(&root.join("website/guide"), "md")?;
    let snippets = grouped(&root.join("snippets/examples"), "rs")?;
    let tests = grouped(&root.join("snippets/tests"), "rs")?;
    let live = read_all(files(&root.join("live/src"), "rs")?, stem)?;
    let live = live.into_values().collect::<Vec<_>>().join("\n");
    Ok(Sources {
        bodies,
        snippets,
        tests,
        live,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn write(root: &Path, relative: &str, text: &str) {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    #[test]
    fn coverage_merges_files() {
        let dir = tempfile::tempdir().unwrap();
        write(
            dir.path(),
            "catalog/coverage/a.yaml",
            "modules:\n  core: {status: gap, reason: r}\n",
        );
        write(
            dir.path(),
            "catalog/coverage/b.yaml",
            "tutorials:\n  t: {status: gap, reason: r}\n",
        );
        let merged = coverage(dir.path()).unwrap();
        assert!(merged.modules.contains_key("core") && merged.tutorials.contains_key("t"));
    }

    #[rstest]
    #[case("modules:\n  core: {status: gap}\n", "module core is already covered")]
    #[case("tutorials:\n  t: {status: gap}\n", "tutorial t is already covered")]
    fn coverage_rejects_duplicates(#[case] second: &str, #[case] expected: &str) {
        let dir = tempfile::tempdir().unwrap();
        let first = "modules:\n  core: {status: gap}\ntutorials:\n  t: {status: gap}\n";
        write(dir.path(), "catalog/coverage/a.yaml", first);
        write(dir.path(), "catalog/coverage/b.yaml", second);
        assert!(coverage(dir.path()).unwrap_err().contains(expected));
    }

    #[test]
    fn missing_directories_are_empty() {
        let dir = tempfile::tempdir().unwrap();
        assert!(coverage(dir.path()).unwrap().modules.is_empty());
        let found = sources(dir.path()).unwrap();
        assert!(found.bodies.is_empty() && found.snippets.is_empty());
    }

    #[test]
    fn sources_read_bodies_and_snippets() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "website/guide/core/tutorial_mat.md", "body");
        write(dir.path(), "website/guide/core/notes.txt", "ignored");
        write(dir.path(), "website/guide/TEMPLATE.md", "not in a section");
        write(
            dir.path(),
            "snippets/examples/core/blend.rs",
            "fn main() {}",
        );
        let found = sources(dir.path()).unwrap();
        assert_eq!(
            found.bodies.keys().collect::<Vec<_>>(),
            ["core/tutorial_mat"]
        );
        assert_eq!(found.snippets["core/blend"], "fn main() {}");
    }

    #[test]
    fn unreadable_files_name_their_path() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "catalog/coverage/a.yaml", "modules: [");
        assert!(coverage(dir.path()).unwrap_err().contains("a.yaml"));
        let unreadable = dir.path().join("website/guide/core/x.md");
        fs::create_dir_all(&unreadable).unwrap();
        assert!(sources(dir.path()).unwrap_err().contains("x.md"));
    }
}
