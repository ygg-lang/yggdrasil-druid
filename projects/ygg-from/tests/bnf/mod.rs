use super::*;

#[test]
pub fn test() -> std::io::Result<()> {
    let cvt = PestConverter::default();
    let out = cvt.parse_pest(include_str!("bnf.bnf"));
    let mut file = File::create("tests/bnf/bnf-raw.ygg")?;
    file.write_all(out.as_bytes())
}