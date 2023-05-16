use clap::Args;

#[derive(Debug, Args)]
pub struct Init {
    #[arg(long)]
    edition: String,
    name: String,
}
