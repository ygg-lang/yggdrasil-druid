use super::*;

#[test]
pub fn test() -> std::io::Result<()> {
    let cvt = PestConverter::default();
    let out = cvt.parse_pest(include_str!("pest.pest"));
    let mut file = File::create("tests/pest/pest.ygg")?;
    file.write_all(out.as_bytes())
}