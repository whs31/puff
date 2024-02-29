use anyhow::ensure;
use colored::Colorize;
use log::{debug, error, info, trace, warn};
use crate::consts::POPPY_REGISTRY_DIRECTORY_NAME;
use crate::manifest::Manifest;
use crate::registry::Registry;
use crate::resolver::DependencyStack;
use crate::utils::environment::Environment;
use crate::utils::global::PROJECT_DIRS;
use crate::utils::helper_types::PlatformArch;

pub struct Poppy
{
  pub config: crate::utils::Config,
  pub registry: Registry,
  pub resolver: DependencyStack,
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
      dirs
        .cache_dir()
        .join(POPPY_REGISTRY_DIRECTORY_NAME)
        .to_str()
        .expect("converting registry path to string slice should never fail")
    );

    let env = match &args.arch {
      Some(x) => {
        let mut env_t = Environment::from_current_environment()?;
        env_t.arch = PlatformArch::from(x.as_str());
        warn!("target platform set to {}", env_t.arch.to_string());
        if env_t.arch == PlatformArch::Unknown {
          error!("unknown platform. you probably misspelled platform name - see list of supported platforms");
          std::process::exit(1);
        }
        env_t
      },
      None => Environment::from_current_environment()?,
    };
    Ok(Self {
      config,
      registry,
      resolver: DependencyStack::new(),
      args,
      env
    })
  }

  pub fn run(&mut self) -> anyhow::Result<()>
  {
    let args = self.args.clone();
    if args.create {
      self.create_manifest()?;
      return Ok(());
    }

    if args.username.is_some() && args.token.is_some() {
      self.setup_oauth(args.username.as_ref().unwrap(), args.token.as_ref().unwrap())?;
    }

    if self.config.auth.username.is_empty() || self.config.auth.token.is_empty() {
      warn!("no username or token provided for artifactory oauth. please provide using --username and --token flags or enter credentials interactively");
      self.input_oauth()?;
    }

    self
      .print_environment()
      .sync(!args.lazy)?
      .install(!args.install)?;
    Ok(())
  }

  fn setup_oauth(&mut self, username: &str, token: &str) -> anyhow::Result<()>
  {
    info!("setting up artifactory oauth for {}", username.bright_yellow());
    ensure!(!username.is_empty(), "username cannot be empty");
    ensure!(!token.is_empty(), "token cannot be empty");
    self.config.auth.username = String::from(username);
    self.config.auth.token = String::from(token);
    self.config.save()?;
    Ok(())
  }

  fn input_oauth(&mut self) -> anyhow::Result<()>
  {
    let mut username = String::new();
    let mut token = String::new();
    info!("enter username:");
    std::io::stdin().read_line(&mut username)?;
    info!("enter token:");
    std::io::stdin().read_line(&mut token)?;
    self.setup_oauth(&username.trim(), &token.trim())
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
    Manifest::from_cli_input()?
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

  pub fn purge()
  {
    warn!("purging config and cache folders");
    let dirs = PROJECT_DIRS.lock().unwrap();
    let config_folder = dirs.config_dir();
    let cache_folder = dirs.cache_dir();
    match std::fs::remove_dir_all(config_folder) {
      Ok(_) => debug!("purged config folder successfully"),
      Err(e) => error!("failed to purge config folder: {}", e)
    }
    match std::fs::remove_dir_all(cache_folder) {
      Ok(_) => debug!("purged cache folder successfully"),
      Err(e) => error!("failed to purge cache folder: {}", e)
    }
    info!("done!");
  }
}