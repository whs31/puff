use std::path::PathBuf;
use std::rc::Rc;
use anyhow::Context;
use colored::Colorize;
use log::{debug, trace, warn};
use crate::artifactory::{Artifactory, SaveAs};
use crate::resolver::Dependency;

pub struct Cache
{
  path: PathBuf,
  artifactory: Rc<Artifactory>
}

impl Cache
{
  pub fn new(path: &str, artifactory: Rc<Artifactory>) -> anyhow::Result<Self>
  {
    if !PathBuf::from(path).exists() {
      std::fs::create_dir_all(path)?
    }
    Ok(Self {
      path: PathBuf::from(path),
      artifactory,
    })
  }

  pub fn check_cache_total_size(&self) -> anyhow::Result<()>
  {
    let total_size = fs_extra::dir::get_size(self.path.as_path())?;
    let ts_mb = total_size as f64 / 1024.0 / 1024.0;
    debug!("local cache total size: {:.1} MB", ts_mb);
    if ts_mb > 512.0 {
      warn!("local cache size is more than 512 MB. it is recommended to clear it manually");
      warn!("run poppy purge --cache to reduce the size of local cache");
    }
    Ok(())
  }

  pub fn get_or_download(&self, dependency: &Dependency, quiet: bool) -> anyhow::Result<PathBuf>
  {
    let tar_name = format!("{}-{}.{}.{}-{}-{}.tar.gz",
      dependency.name,
      dependency.version.major,
      dependency.version.minor,
      dependency.version.patch,
      dependency.arch.to_string(),
      dependency.distribution.to_string()
    );
    if self.contains(dependency) {
      trace!("{} exists in cache", dependency.name.magenta().bold());
      return Ok(self.path
        .join(&dependency.name)
        .join(tar_name)
      )
    }

    self.artifactory
      .pull(dependency, quiet)?
      .save_as(
        &self.path
          .join(&dependency.name)
          .join(&tar_name)
          .as_path()
          .to_str()
          .context("path conversion failed")?
      )?;

    Ok(self.path
      .join(&dependency.name)
      .join(&tar_name)
    )
  }

  pub fn contains(&self, dependency: &Dependency) -> bool
  {
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