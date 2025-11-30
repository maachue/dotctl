use clap::Parser;
use color_eyre::{eyre::Result, owo_colors::OwoColorize};

use crate::cli::Commands;

mod cli;
mod config;
mod runner;
// mod settings_config;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = cli::Cli::parse();
    match args.command {
        Commands::Install {
            config,
            dry_run,
            non_validate,
            debug,
            non_confirm,
            ..
        } => {
            let mut cfg = config::Config::parse(config)?;

            if non_confirm != cfg.options.non_confirm {
                cfg.options.non_confirm = non_confirm;
            }

            if debug {
                println!("{} The config:\n {:?}", "[DEBUG]".red().bold(), cfg);
            }

            runner::manage(&cfg, dry_run, non_validate, cfg.options.non_confirm)?;
        }
        Commands::Set { settings } => {
            println!("{}", settings);
        }
    }
    Ok(())
}
