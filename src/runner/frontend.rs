use crate::runner::BuiltCommand;
use anyhow::{Context, Result};
use which::which;

pub trait FrontEnd {
    fn validate(&self) -> Result<()>;
    fn dry_run(&self);
    fn execute(&self) -> Result<()>;
}

impl FrontEnd for BuiltCommand {
    fn validate(&self) -> Result<()> {
        which(&self.program)
            .with_context(|| format!("Cannot find binary path: {}", self.program))?;
        Ok(())
    }

    fn dry_run(&self) {
        println!("{}", self)
    }

    fn execute(&self) -> Result<()> {
        use duct::cmd;
        if self.is_sudo {
            cmd(
                "sudo",
                std::iter::once(self.program.clone()).chain(self.args.clone()),
            )
            .run()?
        } else {
            cmd(&self.program, &self.args).run()?
        };

        Ok(())
    }
}
