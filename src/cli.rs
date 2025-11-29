use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install {
        config: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    Set {
        settings: String,
    },
}
