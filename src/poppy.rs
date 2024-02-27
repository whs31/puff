use colored::Colorize;
use log::{debug, error, info, trace, warn};
use crate::consts::{POPPY_NAME, POPPY_VERSION};
use crate::utils::ascii::{POPPY_ASCII_ART, POPPY_DOTS_ART};

pub struct Poppy
{
  pub config: crate::utils::Config
}

impl Poppy
{
  pub fn new(config: crate::utils::Config) -> Self
  {
    Self { config }
  }

  pub fn run(&self) -> anyhow::Result<()>
  {
    Ok(())
  }

  pub fn version()
  {
    println!("{}", POPPY_ASCII_ART.yellow().bold());
    println!("{} - package manager!", POPPY_NAME.bright_yellow().bold());
    println!("version: {}", POPPY_VERSION.cyan().bold());
    println!("built from branch: {}",
             option_env!("GIT_BRANCH").unwrap_or("unknown").bold().magenta()
    );
    println!("commit: {}",
             option_env!("GIT_COMMIT").unwrap_or("unknown").bold().magenta()
    );
    println!("dirty: {}",
             option_env!("GIT_DIRTY").unwrap_or("unknown").bold().red()
    );
    println!("build timestamp: {}",
             option_env!("SOURCE_TIMESTAMP").unwrap_or("unknown").green().bold().black()
    );
    println!("{}", "2024 whs31 @ radar-mms (c)".white());
  }
}