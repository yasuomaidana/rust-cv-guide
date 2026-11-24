//! Consistency rules between the catalog, the coverage file and the crate list.

use crate::model::{Catalog, Coverage, Crates, Entry, Sources, Status};
use std::collections::{BTreeMap, BTreeSet};

/// Every way the three files disagree; empty when they are consistent.
pub fn problems(
    catalog: &Catalog,
    coverage: &Coverage,
    crates: &Crates,
    sources: &Sources,
) -> Vec<String> {
    let known: BTreeSet<&str> = crates.crates.iter().map(|c| c.name.as_str()).collect();
    let modules = catalog.modules.iter().map(|m| m.name.as_str());
    let tutorials = catalog.tutorials.iter().map(|t| t.id.as_str());
    let mut out = compare("module", modules, &coverage.modules);
    out.extend(compare("tutorial", tutorials, &coverage.tutorials));
    out.extend(entry_problems("module", &coverage.modules, &known));
    out.extend(entry_problems("tutorial", &coverage.tutorials, &known));
    out.extend(evidence_problems(catalog, coverage, sources));
    out
}

/// What a tutorial's status must be backed by: compiled code and a page, or a reason.
fn evidence_problems(catalog: &Catalog, coverage: &Coverage, sources: &Sources) -> Vec<String> {
    let key = |t: &crate::model::Tutorial| format!("{}/{}", t.section, t.id);
    let keys: BTreeSet<String> = catalog.tutorials.iter().map(key).collect();
    let orphans = sources
        .bodies
        .keys()
        .filter(|k| !keys.contains(*k))
        .map(|k| format!("guide page {k} matches no tutorial"));
    let backed = catalog
        .tutorials
        .iter()
        .filter_map(|t| coverage.tutorials.get(&t.id).map(|e| (t, e)))
        .flat_map(|(t, e)| entry_evidence(&t.id, &key(t), e, sources));
    orphans.chain(backed).collect()
}

fn entry_evidence(id: &str, key: &str, entry: &Entry, sources: &Sources) -> Vec<String> {
    let label = entry.status.label();
    match entry.status {
        Status::Covered | Status::Partial => {
            let page = (!sources.bodies.contains_key(key))
                .then(|| format!("tutorial {id} is {label} but has no guide page {key}.md"));
            snippet_issue(id, entry, sources)
                .into_iter()
                .chain(page)
                .collect()
        }
        Status::Gap | Status::NotPlanned
            if entry.reason.as_deref().is_none_or(|r| r.trim().is_empty()) =>
        {
            vec![format!("tutorial {id} is {label} but gives no reason")]
        }
        _ => Vec::new(),
    }
}

fn snippet_issue(id: &str, entry: &Entry, sources: &Sources) -> Option<String> {
    match entry.snippet.as_deref() {
        None => Some(format!("tutorial {id} names no snippet")),
        Some(s) if !sources.snippets.contains_key(s) => Some(format!(
            "tutorial {id} snippet {s} has no snippets/examples/{s}.rs"
        )),
        Some(_) => None,
    }
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
    use crate::model::fixtures::catalog;
    use crate::model::fixtures::{coverage, coverage_for, crates, entry, full_entry, sources};
    use rstest::rstest;

    #[test]
    fn consistent_files_have_no_problems() {
        let mut cov = coverage_for(full_entry(Status::Covered));
        cov.modules
            .insert("core".into(), entry(Status::Covered, &["ndarray"]));
        assert!(problems(&catalog(), &cov, &crates(), &sources()).is_empty());
    }

    #[rstest]
    #[case(Status::Covered, &["nope"], "tutorial tutorial_mat names unknown crate nope")]
    #[case(Status::Partial, &[], "module core is partial but names no crate")]
    #[case(Status::Covered, &[], "tutorial tutorial_mat is covered but names no crate")]
    fn entry_rules(#[case] status: Status, #[case] names: &[&str], #[case] expected: &str) {
        let found = problems(&catalog(), &coverage(status, names), &crates(), &sources());
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
        let found = problems(&catalog(), &cov, &crates(), &sources());
        assert!(found.contains(&format!("{kind} {removed} has no coverage entry")));
        assert!(found.contains(&format!("coverage names unknown {kind} stray")));
    }

    fn no_snippet() -> Entry {
        Entry {
            snippet: None,
            ..full_entry(Status::Covered)
        }
    }

    fn wrong_snippet() -> Entry {
        Entry {
            snippet: Some("zzz".into()),
            ..full_entry(Status::Partial)
        }
    }

    fn blank_reason(status: Status) -> Entry {
        Entry {
            reason: Some("  ".into()),
            ..entry(status, &[])
        }
    }

    #[rstest]
    #[case(no_snippet(), "tutorial tutorial_mat names no snippet")]
    #[case(
        wrong_snippet(),
        "tutorial tutorial_mat snippet zzz has no snippets/examples/zzz.rs"
    )]
    #[case(entry(Status::Gap, &[]), "tutorial tutorial_mat is gap but gives no reason")]
    #[case(
        blank_reason(Status::NotPlanned),
        "tutorial tutorial_mat is not-planned but gives no reason"
    )]
    fn evidence_rules(#[case] tutorial: Entry, #[case] expected: &str) {
        let found = problems(&catalog(), &coverage_for(tutorial), &crates(), &sources());
        assert!(found.iter().any(|p| p == expected), "{found:?}");
    }

    #[test]
    fn covered_items_need_a_guide_page() {
        let mut none = sources();
        none.bodies.clear();
        let found = problems(
            &catalog(),
            &coverage_for(full_entry(Status::Covered)),
            &crates(),
            &none,
        );
        assert!(
            found.contains(
                &"tutorial tutorial_mat is covered but has no guide page core/tutorial_mat.md"
                    .to_string()
            )
        );
    }

    #[test]
    fn guide_pages_must_match_a_tutorial() {
        let mut extra = sources();
        extra.bodies.insert("core/stray".into(), "x".into());
        let found = problems(
            &catalog(),
            &coverage_for(entry(Status::Unreviewed, &[])),
            &crates(),
            &extra,
        );
        assert!(found.contains(&"guide page core/stray matches no tutorial".to_string()));
    }
}
