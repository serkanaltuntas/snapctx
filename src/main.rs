use clap::Parser;
use snapctx::{cli::Cli, run};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    run(cli)
}