use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;
use snapper_core::SnapperFile;

pub fn load_snapper_file(s: &str) -> Result<SnapperFile> {
    Ok(toml::from_str(s)?)
}

pub fn solc_path(path: &Path, version: &str) -> Result<PathBuf> {
    Ok(path.join(format!("solc-v{}", version)))
}

pub fn default_snapper_outdir() -> Result<PathBuf> {
    let target = env::var("CARGO_TARGET_DIR")?;
    let path = Path::new(&target);
    Ok(path.join("snapper"))
}

pub fn default_snapper_contract_dir() -> Result<PathBuf> {
    Ok(default_snapper_outdir()?.join("contracts"))
}

pub fn default_snapper_artifacts_dir() -> Result<PathBuf> {
    Ok(default_snapper_outdir()?.join("artifacts"))
}

pub fn default_snapper_bins_dir() -> Result<PathBuf> {
    Ok(default_snapper_outdir()?.join("bin"))
}
