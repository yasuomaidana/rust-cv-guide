//! The examples are the coverage tests: a tutorial counts as implemented only when its example runs, is tested, is live in
//! the browser, and its page prints what the test asserts.

use crate::model::{Catalog, Coverage, Entry, Sources};
use std::collections::BTreeSet;

/// Every way an implemented tutorial or an example falls short of that; empty when all are proven.
pub fn problems(catalog: &Catalog, coverage: &Coverage, sources: &Sources) -> Vec<String> {
    let proven = catalog.tutorials.iter().filter_map(|t| {
        let entry = coverage.tutorials.get(&t.id)?;
        let key = format!("{}/{}", t.section, t.id);
        crate::generate::is_implemented(entry.status).then_some((t.id.as_str(), key, entry))
    });
    let mut out: Vec<String> = proven
        .flat_map(|(id, key, entry)| tutorial_problems(id, &key, entry, sources))
        .collect();
    out.extend(orphans(coverage, sources));
    out
}

fn tutorial_problems(id: &str, key: &str, entry: &Entry, sources: &Sources) -> Vec<String> {
    let names = snippet_names(entry);
    let mut out: Vec<String> = names
        .iter()
        .flat_map(|name| example_problems(id, name, sources))
        .collect();
    if let Some(body) = sources.bodies.get(key) {
        out.extend(page_problems(id, body, names.first().copied(), sources));
    }
    out
}

/// The main example first, then each alternative's.
fn snippet_names(entry: &Entry) -> Vec<&str> {
    let main = entry.snippet.as_deref();
    let others = entry.alternatives.iter().map(|a| a.snippet.as_str());
    main.into_iter().chain(others).collect()
}

fn example_problems(id: &str, name: &str, sources: &Sources) -> Vec<String> {
    let untested = (!sources.tests.contains_key(name))
        .then(|| format!("tutorial {id} example {name} has no snippets/tests/{name}.rs"));
    let not_live = (!sources.live.contains(&format!("examples/{name}.rs")))
        .then(|| format!("tutorial {id} example {name} is not compiled into the live crate"));
    untested.into_iter().chain(not_live).collect()
}

fn page_problems(id: &str, body: &str, main: Option<&str>, sources: &Sources) -> Vec<String> {
    let exports = live_functions(body);
    let mut out: Vec<String> = exports
        .iter()
        .filter(|f| !sources.live.contains(&format!("pub fn {f}(")))
        .map(|f| {
            format!("tutorial {id} page runs live function {f}, which cvg-live does not export")
        })
        .collect();
    if exports.is_empty() {
        out.push(format!("tutorial {id} page has no <LiveExample>"));
    }
    out.extend(numbers_problem(id, body, main, sources));
    out
}

/// The function names in `fn="..."` and `fn: "..."` of the page's live examples.
fn live_functions(body: &str) -> Vec<String> {
    let after = |marker: &'static str| {
        body.split(marker)
            .skip(1)
            .filter_map(|rest| rest.split('"').next().map(str::to_owned))
    };
    after("fn=\"").chain(after("fn: \"")).collect()
}

/// The page's printed numbers must be asserted, verbatim, by the main example's test.
fn numbers_problem(id: &str, body: &str, main: Option<&str>, sources: &Sources) -> Option<String> {
    let block = body.split("```text\n").nth(1)?;
    let printed = block.split("```").next().unwrap_or_default().trim_end();
    let asserted = main.and_then(|m| sources.tests.get(m))?;
    let expected = printed.replace('\n', "\\n");
    (!asserted.contains(&expected))
        .then(|| format!("tutorial {id} page prints numbers its test does not assert: {printed}"))
}

fn orphans(coverage: &Coverage, sources: &Sources) -> Vec<String> {
    let named: BTreeSet<&str> = coverage
        .tutorials
        .values()
        .flat_map(snippet_names)
        .collect();
    let stray = sources
        .snippets
        .keys()
        .filter(|k| !named.contains(k.as_str()));
    stray
        .map(|k| format!("example {k} is named by no coverage entry"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Status;
    use crate::model::fixtures::{
        catalog, coverage_for, entry_with_alternative, full_entry, sources,
        sources_with_alternative,
    };
    use rstest::rstest;

    fn check(entry: Entry, sources: &Sources) -> Vec<String> {
        problems(&catalog(), &coverage_for(entry), sources)
    }

    #[test]
    fn a_fully_backed_tutorial_has_no_problems() {
        assert!(check(full_entry(Status::Covered), &sources()).is_empty());
    }

    #[test]
    fn unreviewed_tutorials_need_no_proof() {
        let mut bare = sources();
        bare.tests.clear();
        assert!(
            check(
                Entry {
                    status: Status::Unreviewed,
                    ..full_entry(Status::Covered)
                },
                &bare
            )
            .is_empty()
        );
    }

    fn without_test(s: &mut Sources) {
        s.tests.remove("core/blend");
    }
    fn without_live_include(s: &mut Sources) {
        s.live = "pub fn blend() {}".into();
    }
    fn without_live_page(s: &mut Sources) {
        s.bodies.insert("core/tutorial_mat".into(), "Body\n".into());
    }
    fn with_unexported_function(s: &mut Sources) {
        s.live = s.live.replace("pub fn blend()", "pub fn other()");
    }
    fn with_different_numbers(s: &mut Sources) {
        s.tests.insert(
            "core/blend".into(),
            "assert_eq!(report(), \"b = 2\");".into(),
        );
    }

    #[rstest]
    #[case(
        without_test,
        "tutorial tutorial_mat example core/blend has no snippets/tests/core/blend.rs"
    )]
    #[case(
        without_live_include,
        "tutorial tutorial_mat example core/blend is not compiled into the live crate"
    )]
    #[case(without_live_page, "tutorial tutorial_mat page has no <LiveExample>")]
    #[case(
        with_unexported_function,
        "tutorial tutorial_mat page runs live function blend, which cvg-live does not export"
    )]
    #[case(
        with_different_numbers,
        "tutorial tutorial_mat page prints numbers its test does not assert: a = 1"
    )]
    fn what_is_missing_is_reported(#[case] break_it: fn(&mut Sources), #[case] expected: &str) {
        let mut broken = sources();
        break_it(&mut broken);
        let found = check(full_entry(Status::Covered), &broken);
        assert!(found.iter().any(|p| p == expected), "{found:?}");
    }

    #[test]
    fn alternatives_are_proven_too_and_live_functions_include_alt() {
        let mut s = sources_with_alternative();
        s.bodies.insert(
            "core/tutorial_mat".into(),
            "<LiveExample fn=\"blend\" alt={{fn: \"blend_alt\", label: \"x\"}} />\n".into(),
        );
        let entry = || entry_with_alternative("ndarray", "alt/blend");
        assert!(check(entry(), &s).is_empty());
        s.tests.remove("alt/blend");
        let found = check(entry(), &s);
        assert!(
            found.contains(
                &"tutorial tutorial_mat example alt/blend has no snippets/tests/alt/blend.rs"
                    .to_string()
            )
        );
    }

    #[test]
    fn an_example_no_entry_names_is_an_orphan() {
        let mut s = sources();
        s.snippets
            .insert("core/lonely".into(), "fn main() {}".into());
        let found = check(full_entry(Status::Covered), &s);
        assert!(found.contains(&"example core/lonely is named by no coverage entry".to_string()));
    }
}
