use std::path::PathBuf;
use anyhow::Result;
// use owo_colors::OwoColorize;
use crate::settings_config;

pub fn manage(config: &settings_config::Config, display: bool, non_confirm: bool) -> Result<()> {
    let items = resolve(config);

    for item in &items {
        if display {
            item.display();
        }

        item.validate()?;
        item.change()?;
    }

    Ok(())
}

pub fn resolve(config: &settings_config::Config) -> Vec<BuiltSettings> {
    let mut out = Vec::new();

    let base_str = config
        .options
        .overall_path
        .clone()
        .unwrap_or_else(|| "~/.config/dotctl".into());
    let base = resolve_strpath(&base_str);

    for (master, subs) in &config.settings {
        for (sub, value) in subs {
            let path = build_path(&base, master, sub);
            out.push(BuiltSettings::new(path, master, sub, value));
        }
    }

    out
}

fn resolve_strpath(p: &str) -> PathBuf {
    let expanded = shellexpand::full(p).unwrap();
    let normalized = expanded.replace("/", std::path::MAIN_SEPARATOR_STR.to_string().as_str());
    PathBuf::from(normalized)
}

fn build_path(base: &PathBuf, master: &str, sub: &str) -> PathBuf {
    let mut p = base.clone();

    if master == "overall" {
        p.push(sub);
    } else {
        p.push(master);
        p.push(sub);
    }

    p
}

pub struct BuiltSettings {
    path: PathBuf,
    master: String,
    sub: String,
    value: String,
}

trait FrontEndSet {
    fn change(&self) -> Result<()>;
    fn validate(&self) -> Result<()>;
    fn display(&self);
}

impl FrontEndSet for BuiltSettings {
    fn change(&self) -> Result<()> {
        use std::fs;

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.path, &self.value)?;

        Ok(())
    }

    fn validate(&self) -> Result<()> {
        // TODO validate input  
        // NOTE actually useless
        Ok(())
    }

    fn display(&self) {
        println!("{}", self.show());
        println!("└── Write to: {:?}", self.path);
    }
}

impl BuiltSettings {
    pub fn new(path: PathBuf, master: &str, sub: &str, value: &str) -> Self {
        Self {
            path,
            master: master.to_string(),
            sub: sub.to_string(),
            value: value.to_string(),
        }
    }

    pub fn show(&self) -> String {
        format!("{}.{} = {}", self.master, self.sub, self.value)
    }
}
