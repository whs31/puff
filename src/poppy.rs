use std::path::Path;
use colored::Colorize;
use log::{debug, error, info, trace, warn};
use crate::manifest::Manifest;
use crate::registry;
use crate::registry::Registry;
use crate::utils::environment::Environment;
use crate::utils::global::PROJECT_DIRS;

pub struct Poppy
{
  pub config: crate::utils::Config,
  pub registry: Registry,
  args: crate::Args,
  env: Environment
}

impl Poppy
{
  pub fn new(config: crate::utils::Config, args: crate::Args) -> anyhow::Result<Self>
  {
    let dirs = PROJECT_DIRS.lock().unwrap();
    let registry = Registry::new(
      config.remotes.registry_url.as_str(),
      dirs.cache_dir().to_str().unwrap()
    );
    Ok(Self {
      config,
      registry,
      args,
      env: Environment::from_current_environment()?
    })
  }

  pub fn run(&mut self) -> anyhow::Result<()>
  {
    let args = self.args.clone();
    if args.create {
      self.create_manifest()?;
      return Ok(());
    }
    self
      .print_environment()
      .sync(!args.lazy)?
      .install(!args.install)?;
    Ok(())
  }

  fn print_environment(&mut self) -> &mut Self
  {
    println!();
    debug!("cmake version: {}", &self.env.cmake_version.to_string().magenta());
    debug!("platform: {}", &self.env.arch.to_string().green());
    println!();
    self
  }

  fn sync(&mut self, reclone: bool) -> anyhow::Result<&mut Self>  {
    self.registry.sync(reclone)?;
    Ok(self)
  }

  fn install(&mut self, skip_install: bool) -> anyhow::Result<&mut Self>
  {
    if skip_install {
      warn!("install will be skipped. see --help for more info");
      return Ok(self);
    }
    let manifest = Manifest::from_pwd()?;
    manifest.pretty_print();
    Ok(self)
  }

  fn create_manifest(&self) -> anyhow::Result<()>
  {
    info!("creating new manifest in current working folder");
    let mut manifest = Manifest::from_cli_input()?
      .save()?;
    info!("manifest created successfully");
    Ok(())
  }

  pub fn version()
  {
    println!("{}", crate::utils::ascii::POPPY_ASCII_ART.yellow().bold());
    println!("{} {} version {}",
      crate::consts::POPPY_NAME.bright_yellow().bold(),
      "package manager!".bold(),
      crate::consts::POPPY_VERSION.cyan().bold()
    );

    println!();
    println!("built from branch: {}", option_env!("GIT_BRANCH").unwrap_or("unknown").bold().magenta());
    println!("commit: {}", option_env!("GIT_COMMIT").unwrap_or("unknown").bold().magenta());
    println!("dirty: {}", option_env!("GIT_DIRTY").unwrap_or("unknown").bold().red());
    println!("build timestamp: {}", option_env!("SOURCE_TIMESTAMP").unwrap_or("unknown").green().bold().black());
    println!("copyright {}", "whs31 @ radar-mms (c) 2024".blue().bold());
  }

  pub fn suggest_help()
  {
    error!("incorrect usage. see --help for help!");
    error!("this is unrecoverable error. poppy will shutdown");
    std::process::exit(1);
  }
}