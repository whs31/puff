use std::rc::Rc;
use clap::Parser;
use colored::Colorize;

mod core;
mod types;
mod manifest;
mod utility;
mod names;
mod toolchains;
mod parcel;

fn try_main() -> anyhow::Result<()> {
  let args = Rc::new(core::Args::parse());

  let mut config = core::Config::create_or_load()?;
  config.process_args(&args)?;
  let config = Rc::new(config);

  let env = Rc::new(core::Environment::new(&args)?);
  println!("{}", env.pretty_print()); // todo: move this to install step
  Ok(())
}

fn main() {
  if let Err(e) = try_main() {
    eprintln!("{}: {}",
              "fatal error in parcel".to_string().red().bold(),
              e.to_string().bright_red().bold());
    std::process::exit(1);
  }
}