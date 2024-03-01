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
    if !PathBuf::from(path).exists() {
      std::fs::create_dir_all(path)?
    }
    Ok(Self { path: PathBuf::from(path) })
  }

  pub fn get_or_download(&self, dependency: &Dependency) -> anyhow::Result<PathBuf>
  {
    if self.contains(dependency) {
      trace!("{} exists in cache", dependency.name.magenta().bold());
      let tar_name = format!("{}-{}.{}.{}-{}-{}.tar.gz",
        dependency.name,
        dependency.version.major,
        dependency.version.minor,
        dependency.version.patch,
        dependency.arch.to_string(),
        dependency.distribution.to_string()
      );
      return Ok(self.path
        .join(&dependency.name)
        .join(tar_name)
      )
    }
    Err(anyhow::anyhow!("downloading is not implemented"))
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
    self.path
      .join(&dependency.name)
      .join(tar_name)
      .exists()
  }
}