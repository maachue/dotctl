use anyhow::Result;

use crate::settings::BuiltSettings;

pub trait FrontEndSet {
    fn apply(&self) -> Result<()>;
    fn display(&self);
}

impl FrontEndSet for BuiltSettings {
    fn apply(&self) -> Result<()> {
        use std::fs;

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.path, &self.value)?;

        Ok(())
    }
    fn display(&self) {
        println!("{}", self);
        println!("└── Write to: {:?}", self.path);
    }
}