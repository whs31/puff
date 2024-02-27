use std::path::Path;
use colored::Colorize;
use log::{debug, error, info, trace, warn};
use crate::utils::environment::Environment;
use crate::utils::global::PROJECT_DIRS;

pub struct Poppy
{
  pub config: crate::utils::Config,
  registry_path: String,
  env: Environment
}

impl Poppy
{
  pub fn new(config: crate::utils::Config, args: crate::Args) -> anyhow::Result<Self>
  {
    let dirs = PROJECT_DIRS.lock().unwrap();
    Ok(Self {
      config,
      registry_path: dirs.cache_dir().to_string_lossy().to_string(),
      env: Environment::from_current_environment()?
    })
  }

  pub fn run(&self) -> anyhow::Result<()>
  {
    self
      .print_environment()
      .sync()
    // todo: sync must be conditional
    //Ok(())
  }

  fn print_environment(&self) -> &Self
  {
    debug!("cmake version: {}", self.env.cmake_version.to_string().magenta());
    self
  }

  fn sync(&self) -> anyhow::Result<()>
  {
    info!("syncing with remote repository");
    debug!("syncing into cache ({})", &self.registry_path.dimmed());
    std::fs::create_dir_all(Path::new(&self.registry_path).parent().unwrap())?;
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