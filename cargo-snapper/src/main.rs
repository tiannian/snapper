use clap::Parser;
use commands::Arg;

mod commands;
mod utils;

fn main() {
    env_logger::init();

    let args = Arg::parse();

    if let Err(e) = args.execute() {
        log::error!("{e}");
    }
}
