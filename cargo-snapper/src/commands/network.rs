use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct Network {
    #[arg(long)]
    /// Set the resulting package name, defaults to the `Snapper.toml`
    path: Option<String>,

    #[command(subcommand)]
    subcommand: SubCmd,
}

#[derive(Subcommand, Debug)]
enum SubCmd {
    /// Create a new script.
    Create,
    /// Remove a script
    Remove,
    /// Set network info.
    Set,
    /// Unset network field.
    Unset,
}

impl Network {
    pub fn execute(self) -> Result<()> {
        Ok(())
    }
}
