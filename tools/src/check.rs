//! Consistency rules between the catalog, the coverage file and the crate list.

use crate::model::{Catalog, Coverage, Crates, Entry, Status};
use std::collections::{BTreeMap, BTreeSet};

/// Every way the three files disagree; empty when they are consistent.
pub fn problems(catalog: &Catalog, coverage: &Coverage, crates: &Crates) -> Vec<String> {
    let known: BTreeSet<&str> = crates.crates.iter().map(|c| c.name.as_str()).collect();
    let modules = catalog.modules.iter().map(|m| m.name.as_str());
    let tutorials = catalog.tutorials.iter().map(|t| t.id.as_str());
    let mut out = compare("module", modules, &coverage.modules);
    out.extend(compare("tutorial", tutorials, &coverage.tutorials));
    out.extend(entry_problems("module", &coverage.modules, &known));
    out.extend(entry_problems("tutorial", &coverage.tutorials, &known));
    out
}

fn compare<'a>(
    kind: &str,
    ids: impl Iterator<Item = &'a str>,
    entries: &BTreeMap<String, Entry>,
) -> Vec<String> {
    let ids: BTreeSet<&str> = ids.collect();
    let missing = ids
        .iter()
        .filter(|id| !entries.contains_key(**id))
        .map(|id| format!("{kind} {id} has no coverage entry"));
    let unknown = entries
        .keys()
        .filter(|key| !ids.contains(key.as_str()))
        .map(|key| format!("coverage names unknown {kind} {key}"));
    missing.chain(unknown).collect()
}

fn entry_problems(
    kind: &str,
    entries: &BTreeMap<String, Entry>,
    known: &BTreeSet<&str>,
) -> Vec<String> {
    let mut out = Vec::new();
    for (id, entry) in entries {
        let unknown = entry.crates.iter().filter(|c| !known.contains(c.as_str()));
        out.extend(unknown.map(|c| format!("{kind} {id} names unknown crate {c}")));
        let needs_crate = matches!(entry.status, Status::Covered | Status::Partial);
        if needs_crate && entry.crates.is_empty() {
            out.push(format!(
                "{kind} {id} is {} but names no crate",
                entry.status.label()
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::fixtures::{catalog, coverage, crates, entry};
    use rstest::rstest;

    #[test]
    fn consistent_files_have_no_problems() {
        let cov = coverage(Status::Covered, &["ndarray"]);
        assert!(problems(&catalog(), &cov, &crates()).is_empty());
    }

    #[rstest]
    #[case(Status::Covered, &["nope"], "tutorial tutorial_mat names unknown crate nope")]
    #[case(Status::Partial, &[], "module core is partial but names no crate")]
    #[case(Status::Covered, &[], "tutorial tutorial_mat is covered but names no crate")]
    fn entry_rules(#[case] status: Status, #[case] names: &[&str], #[case] expected: &str) {
        let found = problems(&catalog(), &coverage(status, names), &crates());
        assert!(found.iter().any(|p| p == expected), "{found:?}");
    }

    #[rstest]
    #[case("module")]
    #[case("tutorial")]
    fn missing_and_unknown_entries(#[case] kind: &str) {
        let mut cov = coverage(Status::Gap, &[]);
        let map = if kind == "module" {
            &mut cov.modules
        } else {
            &mut cov.tutorials
        };
        let removed = map.keys().next().cloned().unwrap();
        map.remove(&removed);
        map.insert("stray".into(), entry(Status::Gap, &[]));
        let found = problems(&catalog(), &cov, &crates());
        assert!(found.contains(&format!("{kind} {removed} has no coverage entry")));
        assert!(found.contains(&format!("coverage names unknown {kind} stray")));
    }
}
