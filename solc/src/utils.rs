use std::fs;

use snapper_core::SnapperFile;

use crate::Result;

pub fn load_snapper_file(path: &str) -> Result<SnapperFile> {
    let s = fs::read_to_string(path)?;

    Ok(toml::from_str(&s)?)
}
