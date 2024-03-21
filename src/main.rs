use std::rc::Rc;
use colored::Colorize;

mod core;
mod types;
mod manifest;
mod utility;
mod names;
mod toolchains;

fn try_main() -> anyhow::Result<()> {
  let config = Rc::new(core::Config::create_or_load()?);
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