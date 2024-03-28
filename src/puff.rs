use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;
use anyhow::Context;
use colored::Colorize;
use indicatif::ProgressBar;
use crate::builder::{Builder, Recipe};
use crate::core;
use crate::core::args::{BuildArgs, Command, InstallArgs};
use crate::manifest::Manifest;
use crate::names::PACKED_SOURCE_TARBALL_NAME;
use crate::types::{Arch, Distribution, OperatingSystem};

pub struct Puff
{
  pub config: Rc<core::Config>,
  pub args: Rc<core::Args>,
  pub env: Rc<core::Environment>,
  pub remotes: Rc<crate::artifactory::Registry>,
  pub cache: Rc<crate::cache::Cache>
}

impl Puff
{
  pub fn new(config: Rc<core::Config>, args: Rc<core::Args>, env: Rc<core::Environment>) -> anyhow::Result<Self>
  {
    let remotes = Rc::new(crate::artifactory::Registry::new(config.clone())?);
    let cache = Rc::new(crate::cache::Cache::new(config.clone(), env.clone(), remotes.clone()));
    Ok(Self
    {
      config: config.clone(),
      args,
      env,
      remotes,
      cache
    })
  }

  pub fn pack(&self) -> anyhow::Result<Option<String>>
  {
    let path = match &self.args.command {
      Some(command) => match command {
        Command::Pack(x) => {
          match &x.folder {
            Some(y) => y.clone(),
            None => std::env::current_dir()?.into_os_string().into_string().unwrap(),
          }
        },
        Command::Publish(x) => {
          match &x.folder {
            Some(y) => y.clone(),
            None => std::env::current_dir()?.into_os_string().into_string().unwrap(),
          }
        }
        _ => return Ok(None),
      }
      None => return Ok(None),
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
    Ok(Some(tar_name))
  }

  pub fn publish_sources(&mut self, path: &str, registry_name: &str, force: bool) -> anyhow::Result<&mut Self>
  {
    let remote = self
      .remotes
      .remotes
      .iter()
      .find(|x| x.name == registry_name)
      .context(format!("registry {} not found", registry_name))?;

    remote.push(
      path,
      self.pack()?.as_ref().context("failed to pack sources. contact the maintainer")?,
      Distribution::Sources,
      Arch::Unknown,
      OperatingSystem::Unknown,
      force
    )?;

    Ok(self)
  }

  pub fn sync(&mut self) -> anyhow::Result<&mut Self>
  {
    self.remotes.ping_all()?;
    println!();
    println!();
    Ok(self)
  }

  pub fn install(&mut self, arguments: &InstallArgs) -> anyhow::Result<&mut Self>
  {
    println!("{}", self.env.pretty_print());
    Ok(self)
  }

  pub fn build(&mut self, arguments: &BuildArgs) -> anyhow::Result<&mut Self>
  {
    self.build_packet(arguments.folder.as_ref().unwrap_or(&std::env::current_dir()?.into_os_string().into_string().unwrap()))?;
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