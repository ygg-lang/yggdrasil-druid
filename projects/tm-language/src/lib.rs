mod errors;

use std::collections::BTreeMap;
pub use errors::{Error, Result};
use serde::{Deserialize, Serialize};

mod der;
mod ser;


#[derive(Serialize, Deserialize)]
pub struct TmLanguage {
    #[serde(default)]
    pub information_for_contributors: Vec<String>,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "scopeName")]
    pub scope_name: String,
    #[serde(default)]
    pub patterns: Vec<TmPattern>,
}

pub enum TmPattern {
    Include {
        include: String,
    },
    Complete {
        name: String,
        comment: String,
        begin: String,
        begin_captures: TmCaptures,
        end: String,
        end_captures: TmCaptures,
        patterns: Vec<TmPattern>,
        matches: String,
        captures: TmCaptures,
    },
}

#[derive(Default)]
pub struct TmCaptures {
    inner: BTreeMap<usize, TmPattern>,
}
