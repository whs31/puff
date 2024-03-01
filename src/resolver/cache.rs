use std::path::PathBuf;
use anyhow::Context;
use colored::Colorize;
use log::trace;
use crate::resolver::Dependency;

pub struct Cache
{
  path: PathBuf,
  artifactory_url: String,
  oauth: (String, String)
}

impl Cache
{
  pub fn new(path: &str, artifactory_url: &str, oauth: (&str, &str)) -> anyhow::Result<Self>
  {
    if !PathBuf::from(path).exists() {
      std::fs::create_dir_all(path)?
    }
    Ok(Self {
      path: PathBuf::from(path),
      artifactory_url: artifactory_url.to_string(),
      oauth: (oauth.0.to_string(), oauth.1.to_string())
    })
  }

  pub fn get_or_download(&self, dependency: &Dependency) -> anyhow::Result<PathBuf>
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

    crate::resolver::pull::pull_from_artifactory(
      &dependency,
      &self.artifactory_url,
      self.oauth.0.as_str(),
      self.oauth.1.as_str()
    )?;

    // self.path
    //   .join(&dependency.name)
    //   .join(tar_name)
    //   .as_path()
    //   .to_str()
    //   .context("path conversion failed")?

    Err(anyhow::anyhow!("not implemented"))
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