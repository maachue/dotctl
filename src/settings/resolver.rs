use crate::settings::{BuiltSettings, config::Config};
use anyhow::{Result, bail};

pub fn resolver(
    config: &Config,
    cli: &[&str],
    value_set: &str
) -> Option<BuiltSettings> {
    if cli.len() != 2 {
        return None
    }

    let master = cli[0];
    let sub = cli[1];

    let path = config.options.master_path.clone().unwrap();
    Some(BuiltSettings::new(&path /* should be a &str */, master, sub, value_set))
}

#[derive(Debug)]
pub struct SettingsPath<'a> {
    pub master: &'a str,
    pub sub: &'a str,
}

pub fn resolve_set(input: &str) -> Result<SettingsPath<'_>> {
    let parts: Vec<&str> = input.split('.').collect();

    if parts.len() != 2 {
        bail!("Expected master.sub\nYou entered: {}", input)
    }

    Ok(SettingsPath { master: parts[0], sub: parts[1] })
}