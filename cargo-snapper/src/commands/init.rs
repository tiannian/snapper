use std::{env, process::Command};

use anyhow::Result;
use clap::Args;

use crate::utils::create_workdir;

#[derive(Debug, Args)]
pub struct Init {
    #[arg(long)]
    /// Set the resulting package name, defaults to the directory name
    name: Option<String>,

    #[arg(default_value = ".")]
    path: String,
}

impl Init {
    pub fn execute(self) -> Result<()> {
        let cargo = env::var("CARGO")?;

        let mut cmd = Command::new(cargo);

        if let Some(name) = self.name {
            cmd.arg("--name").arg(name);
        }

        cmd.arg(self.path);

        let result = cmd.status()?;
        if !result.success() {
            log::error!("Failed to execute cargo");

            return Ok(());
        } else {
            log::info!("Create some folder");

            create_workdir()?;
        }

        Ok(())
    }
}
