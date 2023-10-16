use super::*;

#[test]
pub fn test() -> anyhow::Result<()> {
    let cvt = PestConverter::default();
    let out = cvt.convert_pest(include_str!("pest.pest"))?;
    let mut file = File::create("tests/pest/pest.ygg")?;
    file.write_all(out.as_bytes())?;
    Ok(())
}

#[test]
pub fn cvt_semver() -> anyhow::Result<()> {
    let cvt = PestConverter::default();
    let out = cvt.convert_pest(include_str!("semver.pest"))?;
    let mut file = File::create("tests/pest/semver-raw.ygg")?;
    file.write_all(out.as_bytes())?;
    Ok(())
}

#[test]
pub fn cvt_json() -> anyhow::Result<()> {
    let cvt = PestConverter::default();
    let out = cvt.convert_pest(include_str!("json.pest"))?;
    let mut file = File::create("tests/pest/json-raw.ygg")?;
    file.write_all(out.as_bytes())?;
    Ok(())
}
