use std::path::Path;
use colored::Colorize;
use log::{debug, info};
use crate::registry;
use crate::registry::entry::RegistryEntry;

pub struct Registry
{
  pub packages: Vec<RegistryEntry>,
  registry_url: String,
  registry_path: String
}

impl Registry
{
  pub fn new(url: &str, path: &str) -> Self
  {
    Self
    {
      packages: vec![],
      registry_url: String::from(url),
      registry_path: String::from(path)
    }
  }

  pub fn sync(&mut self) -> anyhow::Result<()>
  {
    info!("syncing with remote repository");
    debug!("syncing into cache ({})", &self.registry_path.dimmed());
    std::fs::create_dir_all(Path::new(&self.registry_path).parent().unwrap())?;

    registry::git::clone_repository(
      &self.registry_url,
      &self.registry_path,
      "main" // todo: branch
    )?;
    info!("sync completed");
    Ok(())
  }
}
