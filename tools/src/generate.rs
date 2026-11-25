//! The pages of implemented tutorials, and the roadmap that lists everything else.

use crate::model::{Catalog, Coverage, Entry, Sources, Status, Tutorial};
use crate::roadmap;
use std::collections::BTreeSet;

/// A file to write, relative to `website/`.
pub struct Page {
    pub path: String,
    pub content: String,
}

pub(crate) const BLOB: &str = "https://github.com/opencv/opencv/blob/5.1.0/";
pub(crate) const TREE: &str = "https://github.com/opencv/opencv/tree/5.1.0/";

/// Every generated page: one per implemented tutorial, category labels for sections that have one, and the roadmap.
pub fn pages(catalog: &Catalog, coverage: &Coverage, sources: &Sources) -> Vec<Page> {
    let mut out = Vec::new();
    for t in catalog
        .tutorials
        .iter()
        .filter(|t| implemented(coverage, t))
    {
        let path = format!("docs/opencv/{}/{}.mdx", t.section, t.id);
        let entry = coverage.tutorials.get(&t.id);
        let body = render_body(&format!("{}/{}", t.section, t.id), entry, sources);
        out.push(item_page(path, t, entry, body));
    }
    out.extend(category_pages(catalog, coverage));
    out.push(roadmap::page(catalog, coverage));
    out
}

/// A status with compiled code and a guide page behind it, so the site shows a page for it.
pub(crate) fn is_implemented(status: Status) -> bool {
    matches!(status, Status::Covered | Status::Partial)
}

fn implemented(coverage: &Coverage, tutorial: &Tutorial) -> bool {
    is_implemented(status_of(coverage.tutorials.get(&tutorial.id)).0)
}

pub(crate) fn status_of(entry: Option<&Entry>) -> (Status, &[String]) {
    entry.map_or((Status::Unreviewed, &[]), |e| {
        (e.status, e.crates.as_slice())
    })
}

fn yaml_quote(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

fn code_block(source: &str) -> String {
    format!("```rust\n{}\n```", source.trim_end())
}

/// The compiled example's code; with alternatives, one tab per crate so readers compare the solutions.
fn snippet_markup(entry: Option<&Entry>, sources: &Sources) -> String {
    let Some(entry) = entry else {
        return String::new();
    };
    let main = entry
        .snippet
        .as_ref()
        .and_then(|name| sources.snippets.get(name))
        .map(|source| code_block(source))
        .unwrap_or_default();
    if entry.alternatives.is_empty() {
        return main;
    }
    let others: Vec<&str> = entry
        .alternatives
        .iter()
        .map(|a| a.crate_name.as_str())
        .collect();
    let first: Vec<&str> = entry
        .crates
        .iter()
        .map(String::as_str)
        .filter(|c| !others.contains(c))
        .collect();
    let alternatives = entry.alternatives.iter().map(|a| {
        let code = sources.snippets.get(&a.snippet).map(|s| code_block(s));
        (a.crate_name.clone(), code.unwrap_or_default())
    });
    let tabs = std::iter::once((first.join(" + "), main)).chain(alternatives);
    let items = tabs.enumerate().map(|(i, (label, code))| {
        let default = if i == 0 { " default" } else { "" };
        format!("<TabItem value=\"{label}\" label=\"{label}\"{default}>\n\n{code}\n\n</TabItem>")
    });
    let items: Vec<String> = items.collect();
    format!(
        "<Tabs groupId=\"implementation\">\n{}\n</Tabs>",
        items.join("\n")
    )
}

/// The hand-written guide body with `{{snippet}}` replaced by the compiled example's source.
fn render_body(key: &str, entry: Option<&Entry>, sources: &Sources) -> String {
    let Some(body) = sources.bodies.get(key) else {
        return String::new();
    };
    let code = snippet_markup(entry, sources);
    format!("\n{}\n", body.replace("{{snippet}}", &code).trim_end())
}

/// MDX imports for the tabs, needed only on pages that show alternatives.
fn imports(entry: Option<&Entry>) -> &'static str {
    match entry {
        Some(e) if !e.alternatives.is_empty() => {
            "import Tabs from '@theme/Tabs';\nimport TabItem from '@theme/TabItem';\n\n"
        }
        _ => "",
    }
}

fn item_page(path: String, tutorial: &Tutorial, entry: Option<&Entry>, body: String) -> Page {
    let (status, crates) = status_of(entry);
    let list = if crates.is_empty() {
        "No crate named yet.".to_string()
    } else {
        let link = |c: &String| format!("- [{c}](https://crates.io/crates/{c})");
        crates.iter().map(link).collect::<Vec<_>>().join("\n")
    };
    let reason = entry
        .and_then(|e| e.reason.as_deref())
        .map(|r| format!("\n**Reason:** {r}\n"))
        .unwrap_or_default();
    let content = format!(
        "---\ntitle: {}\n---\n\n{}**Status:** {}\n{reason}\nOpenCV tutorial: [source at 5.1.0]({BLOB}{}).\n\n## Rust crates\n\n{list}\n{body}",
        yaml_quote(&tutorial.title),
        imports(entry),
        status.label(),
        tutorial.source
    );
    Page { path, content }
}

fn category_pages(catalog: &Catalog, coverage: &Coverage) -> Vec<Page> {
    let labels: BTreeSet<&str> = catalog
        .tutorials
        .iter()
        .filter(|t| implemented(coverage, t))
        .map(|t| t.section.as_str())
        .collect();
    let page = |label: &&str| Page {
        path: format!("docs/opencv/{label}/_category_.json"),
        content: format!("{{\"label\": \"{label}\"}}\n"),
    };
    labels.iter().map(page).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::fixtures::{
        catalog, coverage, entry_with_alternative, sources, sources_with_alternative,
    };
    use rstest::rstest;

    fn find<'a>(pages: &'a [Page], path: &str) -> &'a str {
        &pages.iter().find(|p| p.path == path).unwrap().content
    }

    #[rstest]
    #[case(
        "docs/opencv/core/tutorial_mat.mdx",
        "title: \"The \\\"Mat\\\" class\""
    )]
    #[case(
        "docs/opencv/core/tutorial_mat.mdx",
        "- [ndarray](https://crates.io/crates/ndarray)"
    )]
    #[case(
        "docs/opencv/core/tutorial_mat.mdx",
        "blob/5.1.0/doc/tutorials/core/mat.markdown"
    )]
    #[case("docs/opencv/core/_category_.json", "\"label\": \"core\"")]
    #[case("docs/opencv/core/tutorial_mat.mdx", "**Reason:** shown below")]
    #[case(
        "docs/opencv/core/tutorial_mat.mdx",
        "Body.\n\n```rust\nfn main() {}\n```"
    )]
    #[case("docs/roadmap.md", "| covered | 1 | 1 |")]
    fn renders_expected_text(#[case] path: &str, #[case] expected: &str) {
        let mut cov = coverage(Status::Covered, &["ndarray"]);
        let tutorial = cov.tutorials.get_mut("tutorial_mat").unwrap();
        tutorial.snippet = Some("core/blend".into());
        tutorial.reason = Some("shown below".into());
        let pages = pages(&catalog(), &cov, &sources());
        assert!(
            find(&pages, path).contains(expected),
            "{}",
            find(&pages, path)
        );
    }

    #[rstest]
    #[case(Status::Unreviewed)]
    #[case(Status::Gap)]
    #[case(Status::NotPlanned)]
    fn only_implemented_tutorials_get_pages(#[case] status: Status) {
        let pages = pages(&catalog(), &coverage(status, &[]), &Sources::default());
        let paths: Vec<&str> = pages.iter().map(|p| p.path.as_str()).collect();
        assert_eq!(paths, ["docs/roadmap.md"]);
    }

    #[test]
    fn alternatives_become_tabs_with_their_own_code() {
        let mut cov = coverage(Status::Covered, &["ndarray", "oximedia-cv"]);
        let tutorial = cov.tutorials.get_mut("tutorial_mat").unwrap();
        *tutorial = Entry {
            crates: vec!["ndarray".into(), "oximedia-cv".into()],
            ..entry_with_alternative("oximedia-cv", "alt/blend")
        };
        let pages = pages(&catalog(), &cov, &sources_with_alternative());
        let text = find(&pages, "docs/opencv/core/tutorial_mat.mdx");
        for expected in [
            "import Tabs from '@theme/Tabs';",
            "<TabItem value=\"ndarray\" label=\"ndarray\" default>",
            "<TabItem value=\"oximedia-cv\" label=\"oximedia-cv\">",
            "fn alt() {}",
            "fn main() {}",
        ] {
            assert!(text.contains(expected), "{expected}: {text}");
        }
    }

    #[test]
    fn no_entry_means_no_code() {
        assert_eq!(snippet_markup(None, &sources()), "");
    }

    #[test]
    fn a_page_without_crates_says_so() {
        let pages = pages(
            &catalog(),
            &coverage(Status::Partial, &[]),
            &Sources::default(),
        );
        let text = find(&pages, "docs/opencv/core/tutorial_mat.mdx");
        assert!(text.contains("No crate named yet.") && text.contains("**Status:** partial"));
    }
}
