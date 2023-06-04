use std::path::Path;

use anyhow::{anyhow, Result};
use cargo_metadata::{MetadataCommand, Package};
use clap::{Args, Subcommand};

use crate::utils;

#[derive(Debug, Args)]
pub struct Script {
    #[arg(short, long)]
    /// Set the package name
    package: Option<String>,

    #[command(subcommand)]
    subcommand: SubCmd,
}

impl Script {
    pub fn execute(self) -> Result<()> {
        let metadata = MetadataCommand::new().no_deps().exec()?;

        if metadata.packages.len() == 1 {
            let package = &metadata.packages[0];

            self.subcommand.execute(package)?;
        } else {
            let pname = self.package.ok_or(anyhow!("Packages must be set"))?;

            for package in &metadata.packages {
                if package.name == pname {
                    self.subcommand.execute(package)?;
                    return Ok(());
                }
            }

            return Err(anyhow!("Packages not fount"));
        }

        Ok(())
    }
}

#[derive(Subcommand, Debug)]
enum SubCmd {
    /// Create a new script.
    Create(ScriptName),
    /// Remove a script in `Cargo.toml`
    Remove(ScriptName),
}

#[derive(Debug, Args)]
pub struct ScriptName {
    pub name: String,
}

impl SubCmd {
    pub fn execute(self, package: &Package) -> Result<()> {
        let package_path = Path::new(&package.manifest_path)
            .parent()
            .ok_or(anyhow!("Failed goto parent"))?;

        match self {
            Self::Create(n) => utils::scripts::create(package_path, &n.name),
            Self::Remove(n) => utils::scripts::remove(package_path, &n.name),
        }
    }
}
