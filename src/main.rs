use std::rc::Rc;
use clap::Parser;
use colored::Colorize;
use crate::core::args::Command;

mod core;
mod types;
mod manifest;
mod utility;
mod names;
mod toolchains;
mod puff;
mod builder;
mod pack;

fn try_main() -> anyhow::Result<()> {
  let args = Rc::new(core::Args::parse());

  let mut config = core::Config::create_or_load()?;
  config.process_args(&args)?;
  let config = Rc::new(config);
  let env = Rc::new(core::Environment::new(&args)?);

  let mut puff = puff::Puff::new(config, args.clone(), env);

  match &args.command {
    Some(command) => match command {
      Command::Build(x) => { puff.build()?; },
      Command::Pack(x) => { puff.pack()?; },
      _ => {}
    },
    None => {}
  }
  Ok(())
}

fn main() {
  if let Err(e) = try_main() {
    eprintln!("{}: {}",
              "fatal error in puff".to_string().red().bold(),
              e.to_string().bright_red().bold());
    std::process::exit(1);
  }
}