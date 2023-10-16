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
    let mut config = PrettyConfig::default();
    config.struct_names = true;
    file.write_all(to_string_pretty(&rust, config)?.as_bytes())?;
    Ok(())
}