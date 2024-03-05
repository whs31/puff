use std::cell::RefCell;
use std::rc::Rc;
use clap::Parser;
use log::{error, trace};
use crate::args::Args;

mod consts;
mod poppy;
mod utils;
mod args;
mod registry;
mod manifest;
mod resolver;
mod artifactory;

fn main() {
  let args = Args::parse();
  if args.version {
    poppy::Poppy::version();
    std::process::exit(0);
  }

  let quiet = args.manifest_info.is_some() || args.install_path;
  let level = match quiet {
    true => "info",
    false => match args.verbose {
      true => "trace",
      false => "debug",
    }
  };
  utils::cli::init_cli_logger(level)
    .map_err(|e| eprintln!("failed to init cli logger. this is critical error and should never happen ({})", e) )
    .map_err(|_| std::process::exit(1) )
    .unwrap();

  if args.purge {
    poppy::Poppy::purge(args.cache_only);
    std::process::exit(0);
  }

  if args.clean {
    poppy::Poppy::clean()
      .map_err(|e| { error!("failed to clean: {}", e); std::process::exit(1) } )
      .unwrap();
    std::process::exit(0);
  }

  if args.manifest_info.is_some() {
    match poppy::Poppy::manifest_info(args.manifest_info.clone().unwrap().as_str()) {
      Ok(_) => std::process::exit(0),
      Err(e) => {
        error!("failed to grep {}, reason: {}", args.manifest_info.unwrap().as_str(), e);
        std::process::exit(1)
      }
    }
  }

  if args.install_path {
    poppy::Poppy::install_path(args.arch)
      .map_err(|e| { error!("failed to get install path: {}", e); std::process::exit(1) } )
      .unwrap();
    std::process::exit(0);
  }

  if !args.install.clone()
    && !args.sync.clone()
    && !args.create.clone()
    && !args.push.clone().is_some()
  /* todo: other commands */
  {
    poppy::Poppy::suggest_help()
  }

  let mut config = utils::config::Config::create_or_load()
    .expect("failed to load config");
  if let Some(username) = &args.username {
    config.auth.username = username.clone();
    trace!("set username to {}", username.clone());
    config.save()
      .expect("failed to save config");
  }

  if let Some(token) = &args.token {
    config.auth.token = token.clone();
    trace!("set token to {}", token.clone());
    config.save()
      .expect("failed to save config");
  }

  poppy::Poppy::new(
    Rc::new(RefCell::new(config)),
    Rc::new(args)
  )
    .map_err(|e| { error!("fatal error in poppy creation: {}", e); std::process::exit(1) } )
    .unwrap()
    .run()
    .map_err(|e| { error!("fatal error in poppy: {}", e); std::process::exit(1) } )
    .expect("failed to run poppy");
}
