use std::path::PathBuf;

use anyhow::{Result, bail};
use owo_colors::OwoColorize;

use crate::settings::{config::Config, resolver::{SettingsPath, resolver}, utils::build_path};

pub mod config;
pub mod frontend;
pub mod utils;
pub mod resolver;

pub struct BuiltSettings {
    pub path: PathBuf,
    pub master: String,
    pub sub: String,
    pub value: String,
}

impl std::fmt::Display for BuiltSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{} = {}", self.master, self.sub, self.value)
    }
}

impl BuiltSettings {
    pub fn new(path: &str, master: &str, sub: &str, value: &str) -> Self {
        let mut path = crate::utils::resolve_path(&path);
        path = build_path(&path, &master, &sub);
        Self {
            path: path,
            master: master.to_string(),
            sub: sub.to_string(),
            value: value.to_string(),
        }
    }
}

pub fn mange(
    config: &Config,
    cli_tab: SettingsPath,
    set_value: &str,
    display: bool,
    no_confirm: bool,
) -> Result<()> {
    let cli: Vec<&str> = vec![cli_tab.master, cli_tab.sub];
    cli_solver(config, display, no_confirm, &cli, set_value)?;

    Ok(())
}

fn cli_solver(
    config: &Config,
    display: bool,
    no_confirm: bool,
    tab: &Vec<&str>,
    value: &str
) -> Result<()> {
    use crate::settings::frontend::FrontEndSet;
    let item = resolver(config, tab, value).ok_or_else(|| anyhow::anyhow!("Invalid setting name"))?;
    let a = config.settings.get(tab[0]).ok_or_else(|| anyhow::anyhow!("err"))?;
    let _ = a.get(value).ok_or_else(|| anyhow::anyhow!("err"))?; // makes it failed when setting wasn't define

    if display {
        item.display();
    }

    if no_confirm {
        item.apply()?;
    } else {
        item.display();
        if crate::utils::ask("Do you want to update this file?")? {
            item.apply()?;
        }
    }

    Ok(())
}

fn init() -> Result<()> {

    Ok(())
}