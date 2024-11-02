use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the project directory
    pub project_path: PathBuf,

    /// Run in batch mode (no interactive prompts)
    #[arg(long)]
    pub batch_mode: bool,
}