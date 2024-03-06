use std::cell::RefCell;
use std::rc::Rc;
use anyhow::{Context, ensure};
use colored::Colorize;
use log::{debug, error, info, trace, warn};
use crate::args::{Args, Commands, InstallArgs, PurgeArgs};
use crate::artifactory::Artifactory;
use crate::consts::{POPPY_CACHE_DIRECTORY_NAME, POPPY_INSTALLATION_DIRECTORY_NAME, POPPY_REGISTRY_DIRECTORY_NAME};
use crate::manifest::Manifest;
use crate::registry::Registry;
use crate::resolver::{DependencyStack};
use crate::utils::Config;
use crate::utils::environment::{Environment};
use crate::utils::global::PROJECT_DIRS;
use crate::utils::helper_types::{PlatformArch};

pub struct Poppy
{
  pub config: Rc<RefCell<Config>>,
  pub registry: Rc<RefCell<Registry>>,
  pub resolver: Rc<RefCell<DependencyStack>>,
  pub artifactory: Rc<Artifactory>,
  args: Rc<Args>,
  env: Rc<Environment>
}

impl Poppy
{
  pub fn new(config: Rc<RefCell<Config>>, args: Rc<Args>) -> anyhow::Result<Self>
  {
    let dirs = PROJECT_DIRS.lock().unwrap();
    let arch = match &args.command {
      Some(Commands::Install(InstallArgs { arch, .. })) => arch.clone(),
      _ => None
    };
    let env = match arch {
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
    let env = Rc::new(env);
    let artifactory = Rc::new(Artifactory::new(config.clone(), args.clone(), env.clone()));
    let registry = Registry::new(
      config.clone(),
      artifactory.clone(),
      dirs
        .cache_dir()
        .join(POPPY_REGISTRY_DIRECTORY_NAME)
        .to_str()
        .expect("converting registry path to string slice should never fail"),
      args.clone()
    );
    let registry = Rc::new(RefCell::new(registry));

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
      artifactory.clone()
    )?;
    let resolver = Rc::new(RefCell::new(resolver));

    Ok(Self {
      config: config.clone(),
      registry,
      resolver,
      artifactory,
      args,
      env
    })
  }

  pub fn run(&mut self) -> anyhow::Result<()>
  {
    let args = self.args.clone();

    if args.username.is_some() && args.token.is_some() {
      self.setup_oauth(args.username.as_ref().unwrap(), args.token.as_ref().unwrap())?;
    }

    if self.config.borrow().auth.username.is_empty() || self.config.borrow().auth.token.is_empty() {
      warn!("no username or token provided for artifactory oauth. please provide using --username and --token flags or enter credentials interactively");
      self.input_oauth()?;
    }

    match args.command.as_ref().context("command not found")? {
      Commands::Push(x) => {
        ensure!(x.name.is_some(), "name is required for push command");
        self.artifactory
          .push(&Manifest::from_pwd()?, x.name.as_ref().unwrap())?;
        return Ok(());
      }
      _ => {}
    }

    self
      .print_environment()
      .sync(!matches!(args.command, Some(Commands::Install(InstallArgs { lazy: true, .. }))))?
      .fresh(matches!(args.command, Some(Commands::Install(InstallArgs { fresh: true, .. }))))?
      .install()?;
    Ok(())
  }

  fn setup_oauth(&mut self, username: &str, token: &str) -> anyhow::Result<()>
  {
    info!("setting up artifactory oauth for {}", username.bright_yellow());
    ensure!(!username.is_empty(), "username cannot be empty");
    ensure!(!token.is_empty(), "token cannot be empty");
    self.config.borrow_mut().auth.username = String::from(username);
    self.config.borrow_mut().auth.token = String::from(token);
    self.config.borrow().save()?;
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
    self.registry.borrow_mut().sync_aql(!reclone)?;
    Ok(self)
  }

  fn install(&mut self) -> anyhow::Result<&mut Self>
  {
    self.resolver
      .borrow_mut()
      .resolve(self.registry.clone(), self.env.arch.clone())?
      .install_dependencies()?;
    crate::utils::emplace::add_gitignore(
      std::env::current_dir()?
        .join(POPPY_INSTALLATION_DIRECTORY_NAME)
        .to_str()
        .unwrap(),
    )?;
    info!("install completed successfully");
    Ok(self)
  }

  pub fn clean() -> anyhow::Result<()>
  {
    debug!("cleaning up installation directory");
    let path = std::env::current_dir()
      .context("failed to get current directory")?
      .join(POPPY_INSTALLATION_DIRECTORY_NAME)
      .into_os_string()
      .into_string()
      .unwrap();

    if std::fs::remove_dir_all(path.as_str()).is_err() {
      warn!("failed to remove directory: {}", path.as_str());
    }
    Ok(())
  }

  pub fn fresh(&mut self, clean: bool) -> anyhow::Result<&mut Self>
  {
    if clean { Self::clean()?; }
    Ok(self)
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

  pub fn purge(args: &PurgeArgs)
  {
    if !args.all && !args.config && !args.cache {
      error!("nothing to purge. use --all, --config or --cache to purge");
      std::process::exit(1);
    }
    let dirs = PROJECT_DIRS.lock().unwrap();
    let config_folder = dirs.config_dir();
    let cache_folder = dirs.cache_dir();
    if args.cache || args.all {
      warn!("purging cache folder");
      match std::fs::remove_dir_all(cache_folder) {
        Ok(_) => debug!("purged cache folder successfully"),
        Err(e) => trace!("failed to purge cache folder: {}", e)
      }
    }
    if args.config || args.all {
      warn!("purging config folder");
      match std::fs::remove_dir_all(config_folder) {
        Ok(_) => debug!("purged config folder successfully"),
        Err(e) => trace!("failed to purge config folder: {}", e)
      }
    }
    info!("done!");
  }

  pub fn manifest_info(what: &str) -> anyhow::Result<()>
  {
    let manifest = Manifest::from_pwd()?;
    match what {
      "name" => println!("{}", manifest.package.name),
      "version" => println!("{}", manifest.package.version),
      "authors" => println!("{}", manifest.package.authors.context("authors not found")?.join(",")),
      "description" => println!("{}", manifest.package.description.context("description not found")?),
      "wd" => println!("{}", Self::install_path(None)?),
      _ => return Err(anyhow::anyhow!("unknown grep option"))
    }
    Ok(())
  }

  pub fn install_path(arch: Option<String>) -> anyhow::Result<String>
  {
    let env = match arch {
      Some(x) => {
        let mut env_t = Environment::from_current_environment()?;
        env_t.arch = PlatformArch::from(x.as_str());
        warn!("target platform set to {}", env_t.arch.to_string());
        if env_t.arch == PlatformArch::Unknown {
          error!("unknown platform. you probably misspelled platform name - see list of supported platforms");
          return Err(anyhow::anyhow!("unknown platform"))
        }
        env_t
      },
      None => Environment::from_current_environment()?,
    };
    Ok(env.platform_dependent_install_path())
  }
}