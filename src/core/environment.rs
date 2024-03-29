use colored::Colorize;
use crate::{types, core};

pub struct Environment
{
  pub os: types::OperatingSystem,
  pub arch: types::Arch
}

impl Environment
{
  pub fn new(_args: &core::Args) -> anyhow::Result<Self>
  {
    // todo: cross-compilation args
    Ok(Self {
      os: types::OperatingSystem::from_env(),
      arch: types::Arch::from_env()?
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