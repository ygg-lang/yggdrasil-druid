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
            TmPattern::Complete { name, comment, begin, begin_captures, end, end_captures, patterns, matches, captures } => {
                s.serialize_field("comment", comment)?;
                s.serialize_field("name", name)?;
                s.serialize_field("match", matches)?;
                s.serialize_field("patterns", patterns)?;
                s.serialize_field("begin", begin)?;
                s.serialize_field("beginCaptures", begin_captures)?;
                s.serialize_field("end", end)?;
                s.serialize_field("endCaptures", end_captures)?;
                s.serialize_field("captures", captures)?;
            }
        }
        s.end()
    }
}