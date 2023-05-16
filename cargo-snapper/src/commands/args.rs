use clap::{Parser, Subcommand};

use super::Init;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    subcommand: SubCmd,
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
}
