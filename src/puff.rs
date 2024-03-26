use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;
use anyhow::Context;
use colored::Colorize;
use indicatif::ProgressBar;
use crate::builder::{Builder, Recipe};
use crate::core;
use crate::core::args::Command;
use crate::manifest::Manifest;
use crate::names::PACKED_SOURCE_TARBALL_NAME;

pub struct Puff
{
  pub config: Rc<core::Config>,
  pub args: Rc<core::Args>,
  pub env: Rc<core::Environment>
}

impl Puff
{
  pub fn new(config: Rc<core::Config>, args: Rc<core::Args>, env: Rc<core::Environment>) -> Self
  {
    Self
    {
      config,
      args,
      env
    }
  }

  pub fn pack(&mut self) -> anyhow::Result<&mut Self>
  {
    let path = match &self.args.command {
      Some(command) => match command {
        Command::Pack(x) => {
          match &x.folder {
            Some(y) => y.clone(),
            None => std::env::current_dir()?.into_os_string().into_string().unwrap(),
          }
        }
        _ => return Ok(self),
      }
      None => return Ok(self),
    };

    let manifest = Manifest::from_directory(path.as_str())?;
    let _ = Recipe::from_directory(path.as_str())?; // only for checking for it's existence

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(format!("packing {}@{}",
      &manifest.this.name.bold().magenta(),
      &manifest.this.version.to_string().bold().green()
    ));

    let mut fmt: HashMap<String, String> = HashMap::new();
    fmt.insert("name".to_string(), manifest.this.name.clone());
    fmt.insert("version".to_string(), manifest.this.version.clone().to_string());
    let tar_name = strfmt::strfmt(PACKED_SOURCE_TARBALL_NAME, &fmt)
      .context("failed to format tarball name")?;
    crate::pack::pack(path.as_str(), &tar_name)?;

    pb.finish_with_message(format!("{} {}@{}",
      "successfully packed".to_string().green().bold(),
      &manifest.this.name.clone().bold().magenta(),
      &manifest.this.version.clone().to_string().bold().green()
    ));
    Ok(self)
  }

  pub fn install(&mut self) -> anyhow::Result<&mut Self>
  {
    println!("{}", self.env.pretty_print());
    Ok(self)
  }

  pub fn build(&mut self) -> anyhow::Result<&mut Self>
  {
    let path = match &self.args.command {
      Some(command) => match command {
        Command::Build(x) => {
          match &x.folder {
            Some(y) => y.clone(),
            None => std::env::current_dir()?.into_os_string().into_string().unwrap(),
          }
        }
        _ => return Ok(self),
      }
      None => return Ok(self),
    };
    self.build_packet(&path)?;
    Ok(self)
  }

  fn build_packet(&self, path: &str) -> anyhow::Result<()>
  {
    let manifest = Manifest::from_directory(path)?;
    let recipe = Recipe::from_directory(path)?;
    let builder = Builder::new(self.config.clone(), self.env.clone());
    builder.build(&recipe, &manifest, path)?;
    Ok(())
  }
}