use std::path::{PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// installing dependencies based on toml config
    Install {
        /// path to your config
        config: PathBuf,

        /// print all command(s) will execute
        #[arg(long, alias = "validate")]
        dry_run: bool,

        /// non validate run (all will validate by default)
        #[arg(long)]
        non_validate: bool,

        /// debug (print all the config has)
        #[arg(long, default_value_t = false)]
        debug: bool,

        /// skip confirm
        #[arg(long)]
        non_confirm: bool,
    },
    Set {
        settings: String,

        /// debug
        #[arg(long)]
        debug: bool,

        /// config
        #[arg(long, short)]
        config: Option<PathBuf>,
    },
}
