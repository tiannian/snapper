use std::{env, path::PathBuf};

use crate::{CompilerVersions, Config, Result};

pub struct Solc {
    versions: CompilerVersions,
    out_dir: PathBuf,
}

impl Solc {
    pub async fn init() -> Result<Self> {
        let versions = CompilerVersions::load().await?;
        let default_dir = env::var("OUT_DIR")?;
        let out_dir = PathBuf::from(default_dir);
        Ok(Self { versions, out_dir })
    }

    pub async fn new(config: Config) -> Result<Self> {
        let versions = if let Some(upstream) = config.upstream {
            CompilerVersions::load_from(&upstream).await?
        } else {
            CompilerVersions::load().await?
        };

        let out_dir = if let Some(out_dir) = config.out_dir {
            PathBuf::from(out_dir)
        } else {
            let default_dir = env::var("OUT_DIR")?;
            PathBuf::from(default_dir)
        };

        Ok(Self { versions, out_dir })
    }

    pub fn compile() {}
}
