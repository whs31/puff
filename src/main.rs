use clap::Parser;
use log::error;
use crate::args::Args;

mod consts;
mod poppy;
mod utils;
mod args;
mod registry;
mod manifest;

fn main() {
  let args = Args::parse();
  if args.version {
    poppy::Poppy::version();
    std::process::exit(0);
  }

  utils::cli::init_cli_logger("trace")
    .map_err(|e| eprintln!("failed to init cli logger. this is critical error and should never happen ({})", e) )
    .map_err(|_| std::process::exit(1) )
    .unwrap();

  if !args.install && !args.sync && !args.create/* todo: other commands */ {
    poppy::Poppy::suggest_help()
  }

  // todo: tommorow
  // let client = reqwest::blocking::Client::new();
  // let res = client.get("http://192.168.18.1:8082/artifactory/example-repo-local/radar/fmt/fmt-1.0.0-any-sources.tar.gz")
  //   .basic_auth("admin", Some("cmVmdGtuOjAxOjE3NDA1ODU0OTE6czFMaXpDU094b2tOVk5VZnZFcXJKMHo1RjBH"))
  //   .send().unwrap()
  //   .text().unwrap();
  // let file = std::env::current_dir()
  //   .unwrap()
  //   .join("temp-temp")
  //   .into_os_string()
  //   .into_string()
  //   .unwrap();
  // std::fs::write(file, res).unwrap();
  // -------

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
