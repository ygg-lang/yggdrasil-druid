mod errors;

use std::collections::BTreeMap;
pub use errors::{Error, Result};
use serde::{Deserialize, Serialize};



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

#[derive(Serialize, Deserialize)]
pub struct TmPattern {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    pub include: String,
    #[serde(default)]
    pub begin: String,
    #[serde(default)]
    #[serde(rename = "beginCaptures")]
    pub begin_captures: BTreeMap<String, TmPattern>,
    #[serde(default)]
    pub end: Option<String>,
    #[serde(default)]
    #[serde(rename = "endCaptures")]
    pub end_captures: BTreeMap<String, TmPattern>,
    #[serde(default)]
    pub patterns: Vec<TmPattern>,
    #[serde(default)]
    #[serde(rename = "match")]
    pub r#match: String,
    #[serde(default)]
    pub captures: BTreeMap<String, TmPattern>,
}


