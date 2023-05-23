//! Version from upstream

use std::{collections::HashMap, path::Path};

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{Error, Result};

#[derive(Debug, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "windows-amd64")]
    WindowsAmd64,
    #[serde(rename = "linux-amd64")]
    LinuxAmd64,
    #[serde(rename = "macos-amd64")]
    MacOSAmd64,
}

impl Platform {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::LinuxAmd64 => "linux-amd64",
            Self::WindowsAmd64 => "windows-amd64",
            Self::MacOSAmd64 => "macos-amd64",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artifact {
    pub urls: Vec<String>,
    pub keccak256: String,
    pub sha256: String,
}

/// Version on upstream.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerVersions {
    pub builds: HashMap<String, Artifact>,
}

pub const REGISTER_URL: &str =
    "https://raw.githubusercontent.com/tiannian/snapper/main/utils/solidity.json";

impl CompilerVersions {
    /// Load version infomations from upstream
    ///
    /// Default load info from github snapper repo
    pub async fn load() -> Result<Self> {
        Self::load_from(REGISTER_URL).await
    }

    pub async fn load_from(upstream: &str) -> Result<Self> {
        let response = reqwest::get(upstream).await?;

        let res = response.json().await?;

        Ok(res)
    }

    /// Download solc binary
    pub async fn download(&self, version: &str, platform: &Platform, target: &str) -> Result<()> {
        let artifact = self
            .builds
            .get(&format!("{}-{}", version, platform.to_str()))
            .ok_or(Error::NoTargetPlatform)?;

        let mut response = reqwest::get(&artifact.urls[0]).await?.bytes_stream();

        let path = Path::new(target);
        let bin_path = path.join(format!("solc-v{}", version));
        let mut file = File::create(&bin_path).await?;

        while let Some(item) = response.next().await {
            let bytes = item?;

            file.write_all(&bytes).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tokio::runtime::Runtime;

    use crate::CompilerVersions;

    #[test]
    fn test_versions() {
        let runtime = Runtime::new().unwrap();

        runtime.block_on(async move {
            let _artifact = CompilerVersions::load().await.unwrap();

            _artifact
                .download("0.8.20", &super::Platform::LinuxAmd64, "../target")
                .await
                .unwrap()
        });
    }
}
