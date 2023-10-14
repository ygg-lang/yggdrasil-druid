use std::{fs::File, io::prelude::*, path::Path};
use yggdrasil_from::PestConverter;

#[test]
fn ready() {
    println!("it, works!")
}

pub fn convert(input: &str) -> anyhow::Result<()> {
    pest_meta::parse_and_optimize(include_str!("grammar.pest"))?;
    Ok(())
}

#[test]
pub fn test() {
    let cvt = PestConverter::default();
    cvt.parse_pest(include_str!("pest.pest"));
}
