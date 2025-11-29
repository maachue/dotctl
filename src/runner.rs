use anyhow::Result;
use indexmap::IndexMap;
use std::process::Command;
use which::which;

use crate::config::Tasks;

pub struct BuildCommand {
    program: String,
    args: Vec<String>,
    is_sudo: bool,
}

pub fn manage(
    taskmanager: &Option<Vec<String>>,
    tasks: &IndexMap<String, Tasks>,
    pm: &str,
    dry_run: bool,
) -> Result<()> {
    let task_iter: Vec<&Tasks> = match taskmanager {
        Some(order) => order.iter().filter_map(|name| tasks.get(name)).collect(),
        None => tasks.values().collect(),
    };

    for task in task_iter {
        let cmd = match task {
            Tasks::Install { flags, pkgs } | Tasks::Remove { flags, pkgs } => BuildCommand::new(
                pm,
                flags
                    .iter()
                    .cloned()
                    .chain(pkgs.iter().cloned())
                    .collect::<Vec<String>>(),
                true,
            )?,
            Tasks::Update { flags } => BuildCommand::new(pm, flags.to_vec(), true)?,
            Tasks::Shell {
                program,
                flags,
                args,
            } => BuildCommand::new(
                program,
                flags
                    .iter()
                    .cloned()
                    .chain(args.iter().cloned())
                    .collect::<Vec<String>>(),
                false,
            )?,
        };
        if !dry_run {
            BuildCommand::run(&cmd)?;
        } else {
            println!("{}", BuildCommand::display_cmd(&cmd));
        }
    }

    println!("idk what to do right now");

    Ok(())
}

fn validate(program: &str) -> Result<()> {
    which(program)?;
    Ok(())
}

impl BuildCommand {
    pub fn new(program: &str, args: Vec<String>, is_sudo: bool) -> Result<Self> {
        validate(program)?;
        Ok(Self {
            program: program.to_string(),
            args,
            is_sudo,
        })
    }

    pub fn display_cmd(&self) -> String {
        if self.is_sudo {
            format!("sudo {} {}", self.program, self.args.join(" "))
        } else {
            format!("{} {}", self.program, self.args.join(" "))
        }
    }

    pub fn run(&self) -> Result<()> {
        let mut cmd = if self.is_sudo {
            let mut c = Command::new("sudo");
            c.arg(&self.program).args(&self.args);
            c
        } else {
            let mut c = Command::new(&self.program);
            c.args(&self.args);
            c
        };

        let status = cmd.status()?;

        if !status.success() {
            anyhow::bail!("Command failed with status: {}", status);
        }

        Ok(())
    }
}
