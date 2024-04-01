use colored::Colorize;
use crate::{types, core};
use crate::core::args::Command;

pub struct Environment
{
  pub os: types::OperatingSystem,
  pub arch: types::Arch
}

impl Environment
{
  pub fn new(args: &core::Args) -> anyhow::Result<Self>
  {
    let mut os = types::OperatingSystem::from_env();
    let mut arch = types::Arch::from_env()?;
    match &args.command {
      Some(x) => match x {
        Command::Install(y) => {
          if y.os.is_some() { os = y.os.unwrap(); }
          if y.arch.is_some() { arch = y.arch.unwrap(); }
        },
        _ => ()
      },
      _ => ()
    }
    Ok(Self {
      os,
      arch
    })
  }

  pub fn pretty_print(&self) -> String
  {
    format!("target os:   {}\n\
             target arch: {}",
            self.os.to_string().green().bold(),
            self.arch.to_string().yellow().bold()
    )
  }
}