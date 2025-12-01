use clap::Parser;
use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::Commands;

mod cli;
mod config;
mod runner;
mod settings_config;
mod settings;

fn main() -> Result<()> {
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
        Commands::Set { settings, debug, config } => {
            if debug {
                println!("{}", settings)
            }

            let cfg = settings_config::Config::parse(config.unwrap())?;

            if debug {
                println!("{} Config has: {:?}", "[DEBUG]".red().bold(), cfg);
            }

            settings::manage(&cfg, true, true)?;
        }
    }
    Ok(())
}
