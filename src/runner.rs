use std::process::Command;

use anyhow::{Result, bail};
use indexmap::IndexMap;

use crate::config::Tasks;

pub fn runner(
    taskmanager: &Option<Vec<String>>,
    tasks: &IndexMap<String, Tasks>,
    pm: &str,
) -> Result<()> {
    let task_iter: Vec<&Tasks> = match taskmanager {
        Some(order) => order.iter().filter_map(|name| tasks.get(name)).collect(),
        None => tasks.values().collect(),
    };

    for task in task_iter {
        run_task(task, pm)?;
    }

    Ok(())
}

fn run_task(task: &Tasks, pm: &str) -> Result<()> {
    match task {
        Tasks::Install { flags, pkgs } => {
            packagemanager(pm, flags, pkgs)?;
        }
        Tasks::Remove { flags, pkgs } => {
            packagemanager(pm, flags, pkgs)?;
        }
        Tasks::Update { flags } => {
            packagemanager(pm, flags, &[])?;
        }
        Tasks::Shell {
            program,
            flags,
            args,
        } => {
            shellrun(program, flags, args)?;
        }
    }

    Ok(())
}

fn packagemanager(mg: &str, flags: &[String], pkgs: &[String]) -> Result<()> {
    let mut cmd = Command::new("sudo");
    cmd.arg(mg);
    cmd.args(flags);
    cmd.args(pkgs);

    // let status = cmd.status()?;
    // eprintln!("Exit status: {}", status);
    eprintln!("{:?}", cmd);

    Ok(())
}

fn shellrun(program: &str, flags: &[String], args: &[String]) -> Result<()> {
    let mut cmd = Command::new(program);
    cmd.args(flags);
    cmd.args(args);

    // let status = cmd.status()?;
    // eprintln!("Exit status: {}", status);

    eprintln!("{:?}", cmd);

    Ok(())
}
