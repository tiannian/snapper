use clap::Parser;
use commands::Args;

mod commands;
mod utils;

fn main() {
    env_logger::init();

    let args = Args::parse();

    if let Err(e) = args.execute() {
        log::error!("{e}");
    }
}
