use std::rc::Rc;
use clap::Parser;
use colored::Colorize;
use crate::core::args::Command;
use crate::types::Distribution;

mod core;
mod types;
mod manifest;
mod utility;
mod names;
mod toolchains;
mod puff;
mod builder;
mod pack;
mod artifactory;
mod cache;
mod resolver;

fn try_main() -> anyhow::Result<()> {
  let args = Rc::new(core::Args::parse());

  let mut config = core::Config::create_or_load()?;
  config.process_args(&args)?;
  let config = Rc::new(config);
  let env = Rc::new(core::Environment::new(&args)?);

  let mut puff = puff::Puff::new(config, args.clone(), env)?;

  match &args.command {
    Some(command) => match command {
      Command::Build(x) => {
        puff
          .sync()?
          .build(x)?;
      },
      Command::Install(x) => {
        puff
          .sync()?
          .install(x)?;
      },
      Command::Pack(x) => { puff.pack()?; },
      Command::Registry(x) => {
        let _ = puff
          .sync()
          .map_err(|e| eprintln!("{}: {}", "warning".yellow().bold(), e.to_string().yellow().bold()));
      },
      Command::Publish(x) => {
        if let Some(distribution) = &x.dist {
          match distribution {
            Distribution::Sources | Distribution::Unknown => {
              puff
                .sync()?
                .publish_sources(
                  x.folder
                    .as_ref()
                    .unwrap_or(&std::env::current_dir()?.into_os_string().into_string().unwrap())
                    .as_str(),
                  x.name.as_str(),
                  x.force
                )?;
            }
            _ => {
              eprintln!("{}: {}",
                "warning".yellow().bold(),
                "only sources distribution is supported for now".to_string().yellow().bold()
              )
            }
          }
        }
      }
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