use std::{
    fs,
    path::{Path, PathBuf},
};

use snapper_core::SnapperFile;

use crate::Result;

pub fn load_snapper_file(path: &str) -> Result<SnapperFile> {
    let s = fs::read_to_string(path)?;

    Ok(toml::from_str(&s)?)
}

pub fn solc_path(path: &Path, version: &str) -> Result<PathBuf> {
    Ok(path.join(format!("solc-v{}", version)))
}
