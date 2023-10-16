use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::{TmCaptures, TmPattern};

impl Serialize for TmCaptures {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // let mut s = serializer.serialize_map(Some(self.inner.len()))?;
        serializer.collect_map(self.inner.iter())
    }
}

impl Serialize for TmPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("TmPattern", 16)?;
        match self {
            TmPattern::Include { include } => {
                s.serialize_field("include", include)?;
            }
            TmPattern::Complete { name, comment, begin, begin_captures, end, end_captures, matches, captures, patterns } => {
                if !comment.is_empty() {
                    s.serialize_field("comment", comment)?;
                }
                s.serialize_field("name", name)?;
                if !begin.is_empty() {
                    s.serialize_field("begin", begin)?;
                    s.serialize_field("beginCaptures", begin_captures)?;
                }
                if !end.is_empty() {
                    s.serialize_field("begin", begin)?;
                    s.serialize_field("beginCaptures", end_captures)?;
                }
                if !matches.is_empty() {
                    s.serialize_field("match", matches)?;
                    s.serialize_field("captures", captures)?;
                }
                if !patterns.is_empty() {
                    s.serialize_field("patterns", patterns)?;
                }
            }
        }
        s.end()
    }
}