use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use super::Init;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Manage contract project
pub struct Arg {
    #[command(subcommand)]
    subcommand: Snapper,
}

impl Arg {
    pub fn execute(self) -> Result<()> {
        self.subcommand.execute()
    }
}

#[derive(Subcommand, Debug)]
pub enum Snapper {
    Snapper(SnapperRead),
}

impl Snapper {
    pub fn execute(self) -> Result<()> {
        match self {
            Self::Snapper(v) => v.execute(),
        }
    }
}

#[derive(Debug, Args)]
pub struct SnapperRead {
    #[command(subcommand)]
    subcommand: SubCmd,
}

impl SnapperRead {
    pub fn execute(self) -> Result<()> {
        self.subcommand.execute()
    }
}

#[derive(Subcommand, Debug)]
pub enum SubCmd {
    /// Init snapper project in current folder.
    Init(Init),
    /// Create new snapper project.
    New,
    /// Manage scripts.
    Script,
    /// Manage network.
    Network,
    /// Manage library.
    Library,
    /// Configure compiler.
    Compiler,
    /// Start a local node for testing.
    Node,
    /// Show contract ABI
    Abi,
    /// Show Contract Metadata
    Metadata,
}

impl SubCmd {
    pub fn execute(self) -> Result<()> {
        match self {
            Self::Init(v) => v.execute()?,
            _ => {}
        }

        Ok(())
    }
}
