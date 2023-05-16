use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct Script {
    #[arg(long)]
    /// Set the resulting package name, defaults to the `Snapper.toml`
    path: Option<String>,

    #[command(subcommand)]
    subcommand: SubCmd,
}

#[derive(Subcommand, Debug)]
enum SubCmd {
    Add,
    Remove,
    Set,
    Unset,
}

impl Script {
    pub fn execute(self) -> Result<()> {
        Ok(())
    }
}
