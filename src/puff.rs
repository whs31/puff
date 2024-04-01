use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use colored::Colorize;
use crate::core;
use crate::core::args::{BuildArgs, InstallArgs, PurgeArgs};
use crate::names::DEPENDENCIES_FOLDER;
use crate::resolver::{Resolver};
use crate::types::{Arch, Distribution, OperatingSystem};

pub struct Puff
{
  pub config: Rc<core::Config>,
  pub args: Rc<core::Args>,
  pub env: Rc<core::Environment>,
  pub remotes: Rc<RefCell<crate::artifactory::Registry>>,
  pub cache: Rc<crate::cache::Cache>
}

impl Puff
{
  pub fn new(config: Rc<core::Config>, args: Rc<core::Args>, env: Rc<core::Environment>) -> anyhow::Result<Self>
  {
    let remotes = Rc::new(RefCell::new(crate::artifactory::Registry::new(config.clone())?));
    let cache = Rc::new(crate::cache::Cache::new(config.clone(), env.clone(), remotes.clone())?);
    Ok(Self
    {
      config: config.clone(),
      args,
      env,
      remotes,
      cache
    })
  }

  pub fn pack(&self, path: &str) -> anyhow::Result<Option<String>> {
    Ok(Some(crate::pack::pack_with_manifest(path)?))
  }

  pub fn publish_target(
    &self,
    path: &str,
    registry_name: &str,
    force: bool,
    arch: Arch,
    os: OperatingSystem,
    distribution: Distribution
  ) -> anyhow::Result<&Self>
  {
    let remotes_ref = self
      .remotes
      .borrow();
    let remote = remotes_ref
      .remotes
      .iter()
      .find(|x| x.name == registry_name)
      .context(format!("registry {} not found", registry_name))?;

    remote.push(
      path,
      self.pack(path)?.as_ref().context("failed to pack sources. contact the maintainer")?,
      distribution,
      arch,
      os,
      force
    )?;

    Ok(self)
  }

  pub fn publish_sources(&self, path: &str, registry_name: &str, force: bool) -> anyhow::Result<&Self>
  {
    let remotes_ref = self
      .remotes
      .borrow();
    let remote = remotes_ref
      .remotes
      .iter()
      .find(|x| x.name == registry_name)
      .context(format!("registry {} not found", registry_name))?;

    remote.push(
      path,
      self.pack(path)?.as_ref().context("failed to pack sources. contact the maintainer")?,
      Distribution::Sources,
      Arch::Unknown,
      OperatingSystem::Unknown,
      force
    )?;

    Ok(self)
  }

  pub fn sync(&mut self) -> anyhow::Result<&mut Self>
  {
    self.remotes
      .borrow()
      .ping_all()?;
    println!();
    println!();

    self.remotes
      .borrow_mut()
      .sync_all()?;
    Ok(self)
  }

  pub fn install(&mut self, arguments: &InstallArgs) -> anyhow::Result<&mut Self>
  {
    println!("{}", self.env.pretty_print());

    let path = match &arguments.folder {
      Some(x) => x.clone(),
      None => std::env::current_dir()?.into_os_string().into_string().unwrap(),
    };

    if arguments.fresh {
      println!("performing fresh install");
      let dependencies_folder = Path::new(path.as_str()).join(DEPENDENCIES_FOLDER);
      if dependencies_folder.exists() { std::fs::remove_dir_all(dependencies_folder)?; }
    }

    let resolver = Resolver::new(
      self.config.clone(),
      self.env.clone(),
      self.remotes.clone(),
      self.cache.clone()
    );

    resolver
      .resolve(path.as_str())?;
    Ok(self)
  }

  pub fn purge(&self, args: &PurgeArgs) -> anyhow::Result<&Self>
  {
    if args.config || args.all {
      if self.config.directories.dirs.config_dir().exists() {
        println!("purging {} directory", "config".to_string().yellow().bold());
        std::fs::remove_dir_all(self.config.directories.dirs.config_dir())?;
      } else {
        println!("{} directory does not exist", "config".to_string().yellow().bold());
      }
    }
    if args.cache || args.all {
      if self.config.directories.dirs.cache_dir().exists() {
        println!("purging {} directory", "cache".to_string().magenta().bold());
        std::fs::remove_dir_all(self.config.directories.dirs.cache_dir())?;
      } else {
        println!("{} directory does not exist", "cache".to_string().magenta().bold());
      }
    }
    Ok(self)
  }

  // todo: refactor this
  #[allow(dead_code)]
  pub fn build(&mut self, arguments: &BuildArgs) -> anyhow::Result<&mut Self>
  {
    let install_args = InstallArgs {
      folder: arguments.folder.clone(),
      fresh: true
    };
    self
      .install(&install_args)?;
    let _ = Resolver::new(
      self.config.clone(),
      self.env.clone(),
      self.remotes.clone(),
      self.cache.clone()
    );
    Ok(self)
  }
}