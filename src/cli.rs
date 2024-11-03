use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Examples:
    ///   snapctx .                  # current directory
    ///   snapctx ~/projects/myapp   # specific project
    pub project_path: PathBuf,

    /// Run in batch mode (no interactive prompts)
    #[arg(long, help = "Skip interactive prompts and generate summary directly")]
    pub batch_mode: bool,
}