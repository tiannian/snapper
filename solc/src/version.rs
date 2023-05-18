use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerVersions {
    pub builds: HashMap<String, Artifact>,
}

pub const REGISTER_URL: &str =
    "https://raw.githubusercontent.com/tiannian/snapper/main/utils/solidity.json";

impl CompilerVersions {
    pub async fn load() -> Result<Self> {
        let response = reqwest::get(REGISTER_URL).await?;

        let res = response.json().await?;

        Ok(res)
    }

    pub async fn download(&self, version: &str, platform: &Platform) -> Result<()> {
        let artifact = self
            .builds
            .get(&format!("{}-{}", version, platform.to_str()))
            .ok_or(Error::NoTargetPlatform)?;

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
            let _ = CompilerVersions::load().await.unwrap();
        });
    }
}
