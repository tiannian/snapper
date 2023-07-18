use std::{
    env, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{anyhow, Result};
use ethers_contract_abigen::Abigen;
use snapper_core::ProfileType;
use snapper_solc::Solc;

#[derive(Debug, Default)]
pub struct Builder {
    snapper_path: Option<PathBuf>,
    contract_path: Option<PathBuf>,
    bin_path: Option<PathBuf>,
    profile_type: Option<ProfileType>,
}

impl Builder {
    pub fn snapper_path<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.snapper_path = Some(p.as_ref().to_path_buf());
        self
    }

    pub fn contract_path<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.contract_path = Some(p.as_ref().to_path_buf());
        self
    }

    pub fn bin_path<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.bin_path = Some(p.as_ref().to_path_buf());
        self
    }

    fn walk_dir(&self, dir: &Path, solc: &Solc) -> Result<()> {
        if dir.is_dir() {
            let r = fs::read_dir(dir)?;

            for dir in r {
                let entry = dir?;
                let path = entry.path();

                if path.is_dir() {
                    self.walk_dir(&path, solc)?;
                } else {
                    self.compile(&path, solc)?;
                }
            }
        }

        Ok(())
    }

    fn compile(&self, file: &Path, solc: &Solc) -> Result<()> {
        let profile_type = if let Some(p) = &self.profile_type {
            p.clone()
        } else {
            let profile = env::var("PROFILE")?;
            ProfileType::from_str(&profile).map_err(|e| anyhow!("{e}"))?
        };

        let target_dir = temp_path();

        let package_name = if let Some(name) = &solc.snapper.project.rename {
            name.to_string()
        } else {
            env::var("CARGO_PKG_NAME")?
        };

        let filename = file.file_name().ok_or(anyhow!("Failed to get filename"))?;

        let out_dir = target_dir
            .join("snapper")
            .join("artifacts")
            .join(package_name)
            .join(filename);

        let contracts = solc.compile(file, &profile_type, &out_dir)?;

        // Abi generate.
        for c in contracts {
            let abi_path = out_dir.join(format!("{c}.abi"));

            let target_file = env::var("OUT_DIR")?;
            let target_file = Path::new(&target_file).join(filename);
            fs::create_dir_all(&target_file)?;

            let target_file = target_file.join(format!("{c}.rs"));

            Abigen::new(c, abi_path.to_str().ok_or(anyhow!("Failed to get path"))?)
                .map_err(|e| anyhow!("{e}"))?
                .generate()
                .map_err(|e| anyhow!("{e}"))?
                .write_to_file(target_file)?;
        }

        Ok(())
    }

    pub fn build(self) -> Result<()> {
        let snapper_path = if let Some(p) = &self.snapper_path {
            p.clone()
        } else {
            PathBuf::from("Snapper.toml")
        };

        println!("{:?}", snapper_path.canonicalize()?);

        let snapper = fs::read_to_string(snapper_path)?;

        let solc = {
            let bin_path = if let Some(p) = &self.bin_path {
                p.clone()
            } else {
                let target_dir = temp_path();

                target_dir.join("bin")
            };
            Solc::new(bin_path, None, &snapper)?
        };

        // Compile code
        let contract_dir = if let Some(p) = &self.contract_path {
            p.clone()
        } else {
            PathBuf::from("contracts")
        };

        self.walk_dir(&contract_dir, &solc)?;
        Ok(())
    }
}

pub fn build() -> Result<()> {
    let builder = Builder::default();

    builder.build()
}

fn temp_path() -> PathBuf {
    Path::new(env!("OUT_DIR")).to_path_buf()
}
