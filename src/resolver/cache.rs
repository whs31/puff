use std::path::PathBuf;
use colored::Colorize;
use log::trace;
use crate::resolver::Dependency;

pub struct Cache
{
  path: PathBuf
}

impl Cache
{
  pub fn new(path: &str) -> anyhow::Result<Self>
  {
    Ok(Self { path: PathBuf::from(path) })
  }

  pub fn contains(&self, dependency: &Dependency) -> bool
  {
    // http://uav.radar-mms.com/artifactory/poppy-cxx-repo/radar/fmt/fmt-1.0.0-any-sources.tar.gz
    trace!("checking if {} v{} ({}/{}) exists in cache",
      dependency.name.magenta().bold(),
      dependency.version.to_string().green().bold(),
      dependency.distribution.to_string().cyan().bold(),
      dependency.arch.to_string().blue().bold()
    );
    if !self.path.join(&dependency.name).exists() {
      trace!("{} does not exist in cache", dependency.name.magenta().bold());
      return false
    }
    let tar_name = format!("{}-{}.{}.{}-{}-{}.tar.gz",
      dependency.name,
      dependency.version.major,
      dependency.version.minor,
      dependency.version.patch,
      dependency.arch.to_string(),
      dependency.distribution.to_string()
    );
    trace!("checking {tar_name}...");
    self.path.join(tar_name).exists()
  }
}