use clap::Parser;
use log::error;
use crate::args::Args;

mod consts;
mod poppy;
mod utils;
mod args;

fn main() {
  let args = Args::parse();
  if args.version {
    poppy::Poppy::version();
    std::process::exit(0);
  }

  utils::cli::init_cli_logger("trace")
    .map_err(|e| eprintln!("failed to init cli logger. this is critical error and should never happen."))
    .map_err(|_| std::process::exit(1) )
    .unwrap();

  if !args.install && !args.sync /* todo: other commands */ {
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
