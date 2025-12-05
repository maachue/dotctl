use anyhow::{bail, Result};
use owo_colors::OwoColorize;
use crate::settings::{ERR, config::Config};
use super::resolver::SettingsPath;

pub fn validate(config: &Config, key: &SettingsPath) -> Result<()> {
    let master = config
        .settings
        .get(&key.master)
        .ok_or_else(|| anyhow::anyhow!("{} Unknown master key: {}", ERR.bold().red(), key.master))?;

    if !master.contains_key(&key.sub) {
        bail!("{} Unknown setting {}.{}", ERR.bold().red(), key.master, key.sub)
    }

    Ok(())
}
