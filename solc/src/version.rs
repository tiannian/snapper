//! Version from upstream

use std::{collections::HashMap, fs::OpenOptions, path::Path};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

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

    pub fn from_target() -> Option<Self> {
        if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            Some(Self::LinuxAmd64)
        } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
            Some(Self::WindowsAmd64)
        } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
            Some(Self::MacOSAmd64)
        } else {
            None
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
    /// Default load info from ethereum github repo
    pub fn load() -> Result<Self> {
        Self::load_from(REGISTER_URL)
    }

    pub fn load_from(upstream: &str) -> Result<Self> {
        let response = attohttpc::get(upstream).send()?;

        let res = response.json()?;

        Ok(res)
    }

    /// Download solc binary
    pub fn download(&self, version: &str, platform: &Platform, target: &Path) -> Result<()> {
        #[cfg(unix)]
        use std::os::unix::fs::OpenOptionsExt;

        let artifact = self
            .builds
            .get(&format!("{}-{}", version, platform.to_str()))
            .ok_or(anyhow!("No Target support"))?;

        let response = attohttpc::get(&artifact.urls[0]).send()?;

        let mut open = OpenOptions::new();
        open.write(true).create(true);

        #[cfg(unix)]
        open.mode(0o776);

        let file = open.open(target)?;

        response.write_to(file)?;

        Ok(())
    }
}
