use std::fmt::Formatter;
use std::str::FromStr;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, MapAccess, Visitor};
use crate::{TmCaptures, TmPattern};


#[derive(Default)]
struct TmPatternVisitor {
    pub name: String,
    pub comment: String,
    pub include: String,
    pub begin: String,
    pub begin_captures: TmCaptures,
    pub end: String,
    pub end_captures: TmCaptures,
    pub patterns: Vec<TmPattern>,
    pub matches: String,
    pub captures: TmCaptures,
}

impl<'de> Deserialize<'de> for TmPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_any(TmPatternVisitor::default())
    }
}

impl<'de> Visitor<'de> for TmPatternVisitor {
    type Value = TmPattern;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("map of index and rule")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
        let mut value = self;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "name" => value.name = map.next_value()?,
                "comment" => value.comment = map.next_value()?,
                "begin" => value.begin = map.next_value()?,
                "beginCaptures" => value.begin_captures = map.next_value()?,
                "end" => value.end = map.next_value()?,
                "endCaptures" => value.end_captures = map.next_value()?,
                "patterns" => value.patterns = map.next_value()?,
                "match" => value.matches = map.next_value()?,
                "captures" => value.captures = map.next_value()?,
                "include" => value.include = map.next_value()?,
                s => println!("K: {:?}", s)
            }
        }
        Ok(if !value.include.is_empty() {
            TmPattern::Include {
                include: value.include,
            }
        }  else {
            TmPattern::Complete {
                name: value.name,
                comment: value.comment,
                begin: value.begin,
                begin_captures: value.begin_captures,
                end: value.end,
                end_captures: value.end_captures,
                matches: value.matches,
                captures: value.captures,
                patterns: value.patterns,
            }
        })
    }
}

impl<'de> Deserialize<'de> for TmCaptures {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let mut out = TmCaptures::default();
        deserializer.deserialize_any(&mut out)?;
        Ok(out)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_any(place)
    }
}

impl<'i, 'de> Visitor<'de> for &'i mut TmCaptures {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("map of index and rule")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
        while let Some((key, value)) = map.next_entry::<String, TmPattern>()? {
            match usize::from_str(&key) {
                Ok(o) => {
                    self.inner.insert(o, value);
                }
                Err(e) => {
                    Err(A::Error::custom(e))?
                }
            }
        }
        Ok(())
    }
}