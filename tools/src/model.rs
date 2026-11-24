//! Data read from the `catalog/` files.

use serde::Deserialize;
use std::collections::BTreeMap;

/// How well pure-Rust crates cover an OpenCV item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Unreviewed,
    Covered,
    Partial,
    Gap,
    NotPlanned,
}

impl Status {
    /// Every status, in the order the dashboard lists them.
    pub const ALL: [Status; 5] = [
        Status::Covered,
        Status::Partial,
        Status::Gap,
        Status::NotPlanned,
        Status::Unreviewed,
    ];

    /// The kebab-case name used in the YAML files and on the pages.
    pub fn label(self) -> &'static str {
        match self {
            Status::Unreviewed => "unreviewed",
            Status::Covered => "covered",
            Status::Partial => "partial",
            Status::Gap => "gap",
            Status::NotPlanned => "not-planned",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Module {
    pub name: String,
    pub title: String,
    pub source: String,
}

#[derive(Debug, Deserialize)]
pub struct Tutorial {
    pub id: String,
    pub section: String,
    pub title: String,
    pub source: String,
}

/// `catalog/opencv-5.1.yaml`.
#[derive(Debug, Deserialize)]
pub struct Catalog {
    pub modules: Vec<Module>,
    pub tutorials: Vec<Tutorial>,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub status: Status,
    #[serde(default)]
    pub crates: Vec<String>,
}

/// `catalog/coverage.yaml`.
#[derive(Debug, Deserialize)]
pub struct Coverage {
    pub modules: BTreeMap<String, Entry>,
    pub tutorials: BTreeMap<String, Entry>,
}

#[derive(Debug, Deserialize)]
pub struct CrateInfo {
    pub name: String,
}

/// `catalog/crates.yaml`.
#[derive(Debug, Deserialize)]
pub struct Crates {
    pub crates: Vec<CrateInfo>,
}

#[cfg(test)]
pub(crate) mod fixtures {
    use super::*;

    pub fn entry(status: Status, crates: &[&str]) -> Entry {
        Entry {
            status,
            crates: crates.iter().map(|c| c.to_string()).collect(),
        }
    }

    pub fn catalog() -> Catalog {
        Catalog {
            modules: vec![Module {
                name: "core".into(),
                title: "Core".into(),
                source: "modules/core".into(),
            }],
            tutorials: vec![Tutorial {
                id: "tutorial_mat".into(),
                section: "core".into(),
                title: "The \"Mat\" class".into(),
                source: "doc/tutorials/core/mat.markdown".into(),
            }],
        }
    }

    pub fn coverage(status: Status, crates: &[&str]) -> Coverage {
        Coverage {
            modules: BTreeMap::from([("core".to_string(), entry(status, crates))]),
            tutorials: BTreeMap::from([("tutorial_mat".to_string(), entry(status, crates))]),
        }
    }

    pub fn crates() -> Crates {
        Crates {
            crates: vec![CrateInfo {
                name: "ndarray".into(),
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Status::Unreviewed, "unreviewed")]
    #[case(Status::Covered, "covered")]
    #[case(Status::Partial, "partial")]
    #[case(Status::Gap, "gap")]
    #[case(Status::NotPlanned, "not-planned")]
    fn labels_match_yaml_names(#[case] status: Status, #[case] label: &str) {
        assert_eq!(status.label(), label);
        let parsed: Entry = serde_yaml_ng::from_str(&format!("status: {label}")).unwrap();
        assert_eq!(parsed.status, status);
        assert!(Status::ALL.contains(&status));
    }
}
