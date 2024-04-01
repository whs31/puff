use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use clap::Parser;
use colored::Colorize;
use crate::core::args::Command;
use crate::names::{EXPORT_FOLDER, NAME, TARGET_FOLDER, VERSION};
use crate::types::Distribution;
use crate::utility::ascii::ASCII_ART;

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

fn cleanup(target: &str) -> anyhow::Result<()> {
  for entry in std::fs::read_dir(target)? {
    let entry = entry?;
    if entry.file_name().to_str().unwrap().ends_with(".tar.gz") {
      std::fs::remove_file(entry.path())?;
    }
  }
  Ok(())
}

fn try_main() -> anyhow::Result<()> {
  let args = Rc::new(core::Args::parse());

  if args.version {
    println!("{}", ASCII_ART.yellow().bold());
    println!("{} - {} {} {} version {}",
      NAME.to_string().bright_yellow().bold(),
      "a".to_string().bold(),
      "c/c++".to_string().bold().blue(),
      "package manager!".to_string().bold(),
      VERSION.to_string().bold()
    );

    println!();
    println!("built from branch: {}", option_env!("GIT_BRANCH").unwrap_or("unknown").bold().magenta());
    println!("commit: {}", option_env!("GIT_COMMIT").unwrap_or("unknown").bold().magenta());
    println!("dirty: {}", option_env!("GIT_DIRTY").unwrap_or("unknown").bold().red());
    println!("build timestamp: {}", option_env!("SOURCE_TIMESTAMP").unwrap_or("unknown").green().bold().black());
    println!("cli tool by {}", "whs31 <ryazantsev.dl@edu.spbstu.ru>".blue().bold());
    println!("remote/ci server by {}", "spoo0k <mukhin.va@gmail.com>".blue().bold());
    println!("written in rust with love");
    println!("copyright {}", "whs31 © 2024".blue().bold());

    return Ok(());
  }

  let mut config = core::Config::create_or_load()?;
  config.process_args(&args)?;
  let config = Rc::new(config);
  let env = Rc::new(core::Environment::new(&args)?);

  let mut puff = puff::Puff::new(config, args.clone(), env)?;


  match &args.command {
    Some(command) => match command {
      Command::Install(x) => {
        puff
          .sync()?
          .install(x)?;
      },
      Command::Pack(x) => { puff.pack(
        x.folder
          .as_ref()
          .unwrap_or(&std::env::current_dir()?.into_os_string().into_string().unwrap())
          .as_str())?;
      },
      Command::Registry(_x) => {
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
                    .unwrap_or(
                      &std::env::current_dir()?
                        .into_os_string()
                        .into_string()
                        .unwrap()
                    ).as_str(),
                  x.name.as_str(),
                  x.force
                )?;
            },
            _ => {
              let export_folder = Path::new(
                x.folder
                  .as_ref()
                  .unwrap_or(
                    &std::env::current_dir()?
                      .into_os_string()
                      .into_string()
                      .unwrap()
                  )
              ).join(TARGET_FOLDER)
               .join(EXPORT_FOLDER);
              if !export_folder.exists() {
                return Err(anyhow::anyhow!("{}: {}", "error".red().bold(), "target folder does not exist. run 'puff build' first".to_string().red().bold()));
              } else {
                puff
                  .sync()?
                  .publish_target(
                    export_folder.to_str().unwrap(),
                    x.name.as_str(),
                    x.force,
                    x.arch.context("missing architecture argument (--arch)")?,
                    x.os.context("missing operating system argument (--os)")?,
                    x.dist.context("missing distribution argument (--dist)")?,
                  )?;
              }
            }
          }
        } else {
          eprintln!("{}: {}",
            "warning".yellow().bold(),
            "no distribution specified".to_string().yellow().bold()
          )
        }
      },
      Command::Purge(x) => {
        let _ = puff
          .purge(x)
          .map_err(|e| eprintln!("{}: {}", "warning".yellow().bold(), e.to_string().yellow().bold()));
      }
      _ => {}
    },
    None => {}
  }

  if let Some(command) = &args.command {
    match command {
      /* Command::Build(_) | */ Command::Install(_) | Command::Publish(_) => {
        cleanup(
          std::env::current_exe()?
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
        )?
      }
      _ => {}
    }
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