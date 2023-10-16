use std::fs::File;
use std::io::Write;
use ron::ser::{PrettyConfig, to_string_pretty};
use tm_language::TmLanguage;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test() -> anyhow::Result<()> {
    let rust: TmLanguage = json5::from_str(include_str!("rust.tmLanguage.json"))?;
    let mut file = File::create("tests/rust.ron")?;
    file.write_all(to_string_pretty(&rust, PrettyConfig::default())?.as_bytes())?;
    Ok(())
}