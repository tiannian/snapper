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
    pub async fn new_from_path(crate_path: impl AsRef<Path>) -> Result<Self> {
        let config_path = crate_path.as_ref().join("Snapper.toml");

        let p = crate_path.as_ref().to_path_buf();

        let metadata = tokio::task::spawn_blocking(move || {
            MetadataCommand::new().no_deps().current_dir(p).exec()
        })
        .await??;

        let target_path = metadata.target_directory;

        Self::new(config_path, target_path).await
    }

    pub async fn new_from_name(name: &str) -> Result<Self> {
        let metadata =
            tokio::task::spawn_blocking(move || MetadataCommand::new().no_deps().exec()).await??;

        for package in metadata.packages {
            if package.name == name {
                let config_path = package
                    .manifest_path
                    .parent()
                    .ok_or(anyhow!("No parent found"))?
                    .join("Snapper.toml");

                return Self::new(config_path, metadata.target_directory).await;
            }
        }

        Err(anyhow!("No target package found"))
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
