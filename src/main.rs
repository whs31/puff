use clap::Parser;
use log::error;
use crate::args::Args;

mod consts;
mod poppy;
mod utils;
mod args;
mod registry;
mod manifest;
mod resolver;

fn main() {
  let args = Args::parse();
  if args.version {
    poppy::Poppy::version();
    std::process::exit(0);
  }

  let level = match args.manifest_info.is_some() {
    true => "info",
    false => "trace"
  };
  utils::cli::init_cli_logger(level)
    .map_err(|e| eprintln!("failed to init cli logger. this is critical error and should never happen ({})", e) )
    .map_err(|_| std::process::exit(1) )
    .unwrap();

  if args.purge {
    poppy::Poppy::purge(args.cache_only);
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

  if args.push.is_some() {
    poppy::Poppy::push(utils::config::Config::create_or_load()
      .expect("failed to load config"),
      args
        .clone()
    )
      .map_err(|e| { error!("fatal error in poppy push: {}", e); std::process::exit(1) } )
      .unwrap();
    std::process::exit(0);
  }

  if !args.install && !args.sync && !args.create/* todo: other commands */ {
    poppy::Poppy::suggest_help()
  }

  poppy::Poppy::new(utils::config::Config::create_or_load()
    .expect("failed to load config"),
    args
  )
    .map_err(|e| { error!("fatal error in poppy creation: {}", e); std::process::exit(1) } )
    .unwrap()
    .run()
    .map_err(|e| { error!("fatal error in poppy: {}", e); std::process::exit(1) } )
    .expect("failed to run poppy");
}
