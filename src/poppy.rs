use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, ensure};
use colored::Colorize;
use log::{debug, error, info, trace, warn};
use crate::consts::{POPPY_CACHE_DIRECTORY_NAME, POPPY_INSTALLATION_DIRECTORY_NAME, POPPY_REGISTRY_DIRECTORY_NAME};
use crate::manifest::Manifest;
use crate::registry::Registry;
use crate::resolver::{DependencyStack};
use crate::utils::environment::Environment;
use crate::utils::global::PROJECT_DIRS;
use crate::utils::helper_types::{Distribution, PlatformArch};

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
    let resolver = DependencyStack::new(
      dirs
        .cache_dir()
        .join(POPPY_CACHE_DIRECTORY_NAME)
        .to_str()
        .unwrap(),
      std::env::current_dir()
        .context("failed to get current directory")?
        .join(POPPY_INSTALLATION_DIRECTORY_NAME)
        .to_str()
        .context("failed to convert path to string")?,
      config.remotes.artifactory_url.as_str(),
      config.remotes.artifactory_api_url.as_str(),
      (config.auth.username.as_str(), config.auth.token.as_str())
    )?;

    Ok(Self {
      config,
      registry,
      resolver,
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
    self.resolver
      .resolve(&self.registry, self.env.arch.clone())?
      .install_dependencies()?;
    info!("install completed successfully");
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
    println!("cli tool by {}", "whs31 <ryazantsev.dl@edu.spbstu.ru>".blue().bold());
    println!("remote/ci server by {}", "spoo0k <mukhin.va@gmail.com>".blue().bold());
    println!("written in rust with love");
    println!("copyright {}", "whs31 @ radar-mms (c) 2024".blue().bold());
  }

  pub fn suggest_help()
  {
    error!("incorrect usage. see --help for help!");
    error!("this is unrecoverable error. poppy will shutdown");
    std::process::exit(1);
  }

  pub fn purge(cache_only: bool)
  {
    match cache_only {
      true => warn!("purging cache folder only"),
      false => warn!("purging config and cache folders")
    }
    let dirs = PROJECT_DIRS.lock().unwrap();
    let config_folder = dirs.config_dir();
    let cache_folder = dirs.cache_dir();
    if !cache_only {
      match std::fs::remove_dir_all(config_folder) {
        Ok(_) => debug!("purged config folder successfully"),
        Err(e) => error!("failed to purge config folder: {}", e)
      }
    }
    match std::fs::remove_dir_all(cache_folder) {
      Ok(_) => debug!("purged cache folder successfully"),
      Err(e) => error!("failed to purge cache folder: {}", e)
    }
    info!("done!");
  }

  pub fn push(config: crate::utils::Config, args: crate::Args) -> anyhow::Result<()>
  {
    debug!("pushing!");

    ensure!(args.push.as_ref().is_some() && !args.push.as_ref().unwrap().is_empty(), "push target cannot be empty");
    ensure!(args.arch.as_ref().is_some() && !args.arch.as_ref().unwrap().is_empty(), "push arch cannot be empty");
    ensure!(args.distribution.as_ref().is_some() && !args.distribution.as_ref().unwrap().is_empty(), "push distribution cannot be empty");
    ensure!(!config.auth.username.is_empty() && !config.auth.token.is_empty(),
      "no username or token provided for artifactory oauth. please provide using --username and \
      --token flags or enter credentials interactively via poppy --sync");

    let arch = PlatformArch::from(args.arch.unwrap().as_str());
    let distribution = Distribution::from(args.distribution.unwrap().as_str());
    let push_target = args.push.context("empty push target!")?;

    trace!("artifactory base url: {}", &config.remotes.artifactory_url);

    let manifest = Manifest::from_pwd()?;

    debug!("package name: {}", manifest.package.name.yellow().bold());
    debug!("package version: {}", manifest.package.version.to_string().cyan().bold());
    debug!("package tarball: {}", &push_target.purple().bold());
    debug!("arch: {}", &arch.to_string().green().bold());
    debug!("distribution: {}", &distribution.to_string().magenta().bold());

    let mut fmt: HashMap<String, String> = HashMap::new();
    fmt.insert("name".to_string(), manifest.package.name.clone());
    fmt.insert("major".to_string(), manifest.package.version.clone().major.to_string());
    fmt.insert("minor".to_string(), manifest.package.version.clone().minor.to_string());
    fmt.insert("patch".to_string(), manifest.package.version.clone().patch.to_string());
    fmt.insert("arch".to_string(), arch.clone().to_string());
    fmt.insert("distribution".to_string(), distribution.clone().to_string());

    let url = strfmt::strfmt(&config.remotes.artifactory_url, &fmt)
      .context("failed to format artifactory url")?;
    //debug!("artifactory prepared url: {}", url);

    crate::resolver::push::push_to_artifactory(
      url.as_str(),
      crate::resolver::push::tar_to_binary(
        Path::new(push_target.as_str()).to_str().context(
          "failed to convert push target to string"
        )?
      )?.as_slice(),
      config.auth.username.as_str(),
      config.auth.token.as_str()
    )?;

    info!("pushing done!");
    Ok(())
  }

  pub fn manifest_info(what: &str) -> anyhow::Result<()>
  {
    let manifest = Manifest::from_pwd()?;
    match what {
      "name" => println!("{}", manifest.package.name),
      "version" => println!("{}", manifest.package.version),
      "authors" => println!("{}", manifest.package.authors.context("authors not found")?.join(",")),
      "description" => println!("{}", manifest.package.description.context("description not found")?),
      _ => return Err(anyhow::anyhow!("unknown grep option"))
    }
    Ok(())
  }
}