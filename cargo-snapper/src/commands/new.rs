use std::{env, path::Path, process::Command};

use anyhow::Result;
use clap::Args;

use crate::utils;

#[derive(Debug, Args)]
pub struct New {
    #[arg(long)]
    /// Set the resulting package name, defaults to the directory name
    name: Option<String>,

    path: String,
}

impl New {
    pub fn execute(self) -> Result<()> {
        let cargo = env::var("CARGO")?;

        let mut cmd = Command::new(cargo);

        cmd.arg("new").arg("--lib");

        if let Some(name) = self.name {
            cmd.arg("--name").arg(name);
        }

        cmd.arg(&self.path);

        let result = cmd.status()?;
        if !result.success() {
            log::error!("Failed to execute cargo");

            return Ok(());
        } else {
            log::info!("Create some folder");

            utils::project::create(Path::new(&self.path))?;
        }

        Ok(())
    }
}
