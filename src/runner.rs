use cin::cin::cin;
use anyhow::{Result, bail};
use owo_colors::OwoColorize;
use indexmap::IndexMap;
use std::process::Command;
use which::which;

use crate::config::{Config, Tasks};

pub struct BuildCommand {
    program: String,
    args: Vec<String>,
    is_sudo: bool,
}

pub fn manage(config: &Config, dry_run: bool, non_validate: bool, non_cofirm: bool) -> Result<()> {
    let list = resolve_tasks(&config.taskmanager.run, &config.tasks);

    let cmds: Vec<BuildCommand> = list
        .iter()
        .map(|t| t.into_cmd(&config.options.packagemanager))
        .collect();

    if dry_run {
        println!("{} The command(s) will exectute:", "[INFO]".cyan().bold());
    }

    for cmd in cmds {
        if dry_run {
            if ! non_validate {
                cmd.validate()?;
            }
            cmd.dry_run();
        }
        else {
            if ! non_validate {
                cmd.validate()?;
            }

            if non_cofirm {
                    cmd.execute()?;
            } else {
                println!("{} The command will execute:", "[INFO]".cyan().bold());
                cmd.dry_run();

                let choice: String = cin::<String>("Do you want to execute this command? (y/n/a|all): ")
                    .trim()
                    .to_lowercase();

                match choice.as_str() {
                    "y" => cmd.execute()?,
                    "n" => println!("{} User terminated the command {}", "[ERR]".red(), cmd.display_cmd().yellow()),
                    "a"
                    | "all"
                    => bail!("{} User terminated", "[ERR]".red()),
                    _ => bail!("{} Choice invalid!", "[ERR]".red()),
                }
            }
        }
    }

    Ok(())
}

pub fn resolve_tasks<'a>(
    order: &Option<Vec<String>>,
    tasks: &'a IndexMap<String, Tasks>,
) -> Vec<&'a Tasks> {
    match order {
        Some(list) => list
            .iter()
            .filter_map(|name| tasks.get(name))
            .collect::<Vec<_>>(),

        None => tasks.values().collect::<Vec<_>>(),
    }
}

trait Executable {
    fn validate(&self) -> Result<()>;
    fn dry_run(&self);
    fn execute(&self) -> Result<()>;
}

impl Executable for BuildCommand {
    fn validate(&self) -> Result<()> {
        match which(&self.program) {
            Ok(_) => {},
            Err(_) => bail!("cannot find binary path: {}", &self.program),
        };
        Ok(())
    }
    fn dry_run(&self) {
        println!("{}", self.display_cmd());
    }
    fn execute(&self) -> Result<()> {
        let status = if self.is_sudo {
            Command::new("sudo")
                .arg(&self.program)
                .args(&self.args)
                .status()?
        } else {
            Command::new(&self.program).args(&self.args).status()?
        };

        if ! status.success() {
            bail!("{} Command failed: {}", "[ERROR]".red(), self.display_cmd().yellow());
        }

        Ok(())
    }
}

impl Tasks {
    pub fn into_cmd(&self, pm: &str) -> BuildCommand {
        match self {
            Tasks::Install {
                flags,
                pkgs,
                is_sudo,
            }
            | Tasks::Remove {
                flags,
                pkgs,
                is_sudo,
            } => BuildCommand::new(
                pm,
                flags.iter().cloned().chain(pkgs.iter().cloned()).collect(),
                *is_sudo,
            ),
            Tasks::Update { flags, is_sudo } => BuildCommand::new(pm, flags.to_vec(), *is_sudo),
            Tasks::Shell {
                program,
                flags,
                args,
                is_sudo,
            } => BuildCommand::new(
                &program,
                flags.iter().cloned().chain(args.iter().cloned()).collect(),
                *is_sudo,
            ),
        }
    }
}

impl BuildCommand {
    pub fn new(program: &str, args: Vec<String>, is_sudo: bool) -> Self {
        Self {
            program: program.to_string(),
            args,
            is_sudo,
        }
    }

    pub fn display_cmd(&self) -> String {
        if self.is_sudo {
            format!("sudo {} {}", self.program, self.args.join(" "))
        } else {
            format!("{} {}", self.program, self.args.join(" "))
        }
    }
}