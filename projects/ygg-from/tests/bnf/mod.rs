
use super::*;

#[test]
pub fn test_bnf() -> anyhow::Result<()> {
    let cvt = BNFConverter::default();
    let out = cvt.convert_bnf(include_str!("bnf.bnf"))?;
    let path = Path::new("tests/bnf/bnf-raw.ygg").canonicalize()?;
    println!("{}", Url::from_file_path(&path).unwrap());
    let mut file = File::create(&path)?;
    file.write_all(out.as_bytes())?;

    Ok(())
}