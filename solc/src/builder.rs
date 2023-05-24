use std::{collections::HashMap, env, path::PathBuf};

use snapper_core::SnapperFile;
use tokio::fs;

use crate::{
    input::{self, SourceFile},
    utils,
    version::Platform,
    CompilerInput, CompilerVersions, Config, Error, Result,
};

pub struct Solc {
    out_dir: PathBuf,
    snapper: SnapperFile,
}

impl Solc {
    /// New a solc instance
    ///
    /// Notice: this function only can be call in `build.rs`
    pub async fn new() -> Result<Self> {
        let versions = CompilerVersions::load().await?;
        let default_dir = env::var("OUT_DIR")?;
        let out_dir = PathBuf::from(default_dir);
        let snapper = utils::load_snapper_file(".")?;

        let platform = Platform::from_target().ok_or(Error::UnsupportPlatform)?;

        for version in &snapper.solidity.version {
            versions.download(&version, &platform, &out_dir).await?;
        }

        Ok(Self { out_dir, snapper })
    }

    pub async fn new_with_config(config: Config) -> Result<Self> {
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

        let snapper = if let Some(snapper) = config.snapper_file {
            utils::load_snapper_file(&snapper)?
        } else {
            utils::load_snapper_file(".")?
        };

        let platform = Platform::from_target().ok_or(Error::UnsupportPlatform)?;

        for version in &snapper.solidity.version {
            versions.download(&version, &platform, &out_dir).await?;
        }

        Ok(Self { out_dir, snapper })
    }

    pub async fn compile(self) -> Result<()> {
        let mut all_files = fs::read_dir(".").await?;

        let mut sources = HashMap::new();

        while let Some(file) = all_files.next_entry().await? {
            if file.file_type().await?.is_file() {
                let path = file.path();
                let filename = file.file_name().to_string_lossy().to_string();

                let sf = SourceFile {
                    keccak256: None,
                    urls: vec![path.to_string_lossy().to_string()],
                };
                sources.insert(filename, sf);
            }
        }

        // let input = CompilerInput {
        //     language: input::InputLanguage::Solidity,
        //     sources,
        //     settings: input::Settings {
        //         stop_after: input::StopAfter::Parsing,
        //         remappings: vec![],
        //         optimizer: (),
        //         evm_version: (),
        //         via_ir: self.snapper.solidity.via_ir,
        //         debug: (),
        //         metadata: (),
        //         libraries: (),
        //         output_selection: (),
        //         model_checker: None,
        //     },
        // };

        Ok(())
    }
}
