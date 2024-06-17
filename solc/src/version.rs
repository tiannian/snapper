//! Version from upstream

use std::{collections::BTreeMap, fs::OpenOptions, path::Path};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use snapper_core::Platform;

#[derive(Debug, Serialize, Deserialize)]
struct List {
    pub releases: Releases,
}

type Releases = BTreeMap<String, String>;

/// Version on upstream.
#[derive(Debug)]
pub struct CompilerVersions {
    linux_amd64: Releases,
    windows_amd64: Releases,
    macosx_amd64: Releases,

    base: String,
}

pub const REGISTER_BASE_URL: &str = "https://binaries.soliditylang.org";

fn load_list(base: &str, platform: &Platform) -> Result<List> {
    let url = format!("{}/{}/list.json", base, platform.to_str());

    println!("{}", url);

    let response = attohttpc::get(url).send()?;

    Ok(response.json()?)
}

impl CompilerVersions {
    /// Load version infomations from upstream
    ///
    /// Default load info from ethereum github repo
    pub fn load() -> Result<Self> {
        Self::load_from(REGISTER_BASE_URL)
    }

    pub fn load_from(base: &str) -> Result<Self> {
        let linux_amd64 = load_list(base, &Platform::LinuxAmd64)?.releases;
        let windows_amd64 = load_list(base, &Platform::WindowsAmd64)?.releases;
        let macosx_amd64 = load_list(base, &Platform::MacOSAmd64)?.releases;

        Ok(Self {
            linux_amd64,
            windows_amd64,
            macosx_amd64,

            base: base.into(),
        })
    }

    /// Download solc binary
    pub fn download(&self, version: &str, platform: &Platform, target: &Path) -> Result<()> {
        #[cfg(unix)]
        use std::os::unix::fs::OpenOptionsExt;

        let releases = match platform {
            Platform::LinuxAmd64 => &self.linux_amd64,
            Platform::MacOSAmd64 => &self.macosx_amd64,
            Platform::WindowsAmd64 => &self.windows_amd64,
        };

        let filename = releases
            .get(version)
            .ok_or(anyhow!("No target solc version"))?;

        let url = format!("{}/{}/{}", self.base, platform.to_str(), filename);

        let response = attohttpc::get(url).send()?;

        let mut open = OpenOptions::new();
        open.write(true).create(true);

        #[cfg(unix)]
        open.mode(0o776);

        let file = open.open(target)?;

        response.write_to(file)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::CompilerVersions;

    #[test]
    fn test_load() {
        let versions = CompilerVersions::load().unwrap();

        println!("{:?}", versions);
    }
}
