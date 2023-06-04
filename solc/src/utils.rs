use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use snapper_core::SnapperFile;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{output::Selector, Result};

pub fn load_snapper_file(path: &str) -> Result<SnapperFile> {
    let s = fs::read_to_string(path)?;

    Ok(toml::from_str(&s)?)
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

pub async fn write_method_identifiers(path: &Path, mi: &HashMap<String, Selector>) -> Result<()> {
    let mut file = File::create(path).await?;

    for (name, selector) in mi {
        let s = &selector.value;
        file.write_all(s).await?;
        file.write_all(name.as_bytes()).await?;
        file.write_all("\n".as_bytes()).await?;
    }

    Ok(())
}
