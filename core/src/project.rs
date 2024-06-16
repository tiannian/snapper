use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use cargo_metadata::MetadataCommand;
use tokio::fs;

use crate::{Platform, SnapperConfig};

pub struct SnapperPackage {
    config: SnapperConfig,

    snapper_outdir: PathBuf,
    package_dir: PathBuf,

    platform: Platform,
}

impl SnapperPackage {
    /// Create Snapper Package.
    pub async fn new(config_path: impl AsRef<Path>, target_path: impl AsRef<Path>) -> Result<Self> {
        let config = fs::read_to_string(&config_path).await?;
        let config = toml::from_str(&config)?;

        let snapper_outdir = target_path.as_ref().join("snapper");

        Ok(Self {
            config,
            snapper_outdir,
            package_dir: config_path
                .as_ref()
                .parent()
                .ok_or(anyhow!("Failed to get parent of config"))?
                .into(),
            platform: Platform::from_target().ok_or(anyhow!("Unsopported target"))?,
        })
    }

    /// Crate Snapper package from environment
    pub async fn new_from_env(crate_path: impl AsRef<Path>) -> Result<Self> {
        let config_path = crate_path.as_ref().join("Snapper.toml");

        let metadata = MetadataCommand::new()
            .no_deps()
            .current_dir(crate_path.as_ref())
            .exec()?;

        let target_path = metadata.target_directory;

        Self::new(config_path, target_path).await
    }

    /// Config of Snapper Package
    pub fn config(&self) -> &SnapperConfig {
        &self.config
    }

    /// Output dir of package.
    pub fn outdir(&self) -> &Path {
        &self.snapper_outdir
    }

    /// contracts dir
    pub fn contracts_dir(&self) -> PathBuf {
        self.package_dir.join(&self.config.project.contracts)
    }

    /// binary dir
    pub fn bin_dir(&self) -> PathBuf {
        self.outdir().join("bin")
    }

    /// solc path based on platform and version
    pub fn solc_path(&self) -> PathBuf {
        self.bin_dir().join(format!(
            "solc-{}-{}",
            self.platform.to_str(),
            self.config.solidity.version
        ))
    }
}
