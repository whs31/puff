use std::cell::RefCell;
use std::rc::Rc;
use clap::Parser;
use log::{error, trace};
use crate::args::{Args, Commands};

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

  utils::cli::init_cli_logger(
    match &args.command {
      Some(Commands::Parse { .. })=> "error",
      _ => match args.verbose {
        true => "trace",
        false => "debug",
      }
    }
  )
    .map_err(|e| eprintln!("failed to init cli logger. this is critical error and should never happen ({})", e) )
    .map_err(|_| std::process::exit(1) )
    .unwrap();

  match args.command
  {
    Some(Commands::Install) | Some(Commands::Push { .. }) => {}
    Some(Commands::Clean) => {
      poppy::Poppy::clean()
        .map_err(|e| { error!("failed to clean: {}", e); std::process::exit(1) } )
        .unwrap();
      std::process::exit(0);
    }
    Some(Commands::Purge) => {
      poppy::Poppy::purge(args.cache_only);
      std::process::exit(0);
    },
    Some(Commands::Parse { what }) => {
      if what.is_some() {
        poppy::Poppy::manifest_info(what.unwrap().as_str())
          .map_err(|e| { error!("failed to parse: {}", e); std::process::exit(1) } )
          .unwrap();
        std::process::exit(0);
      } else {
        error!("no parsing expression provided. read poppy --help.");
        std::process::exit(1);
      }
    },
    None => {
      error!("no command provided. read poppy --help.");
      std::process::exit(1);
    }
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
