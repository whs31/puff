use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;
use anyhow::{anyhow, bail, Context};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressFinish};
use crate::core;
use crate::resolver::{Dependency, PackageGet};

pub struct Cache
{
  pub config: Rc<core::Config>,
  pub env: Rc<core::Environment>,
  pub registry: Rc<RefCell<crate::artifactory::Registry>>,
  pub path: PathBuf
}

impl Cache
{
  pub fn new(config: Rc<core::Config>, env: Rc<core::Environment>, registry: Rc<RefCell<crate::artifactory::Registry>>) -> anyhow::Result<Self>
  {
    let path = config.directories.dirs.cache_dir().to_path_buf();
    std::fs::create_dir_all(&path)?;
    Ok(Self
    {
      config,
      env,
      registry,
      path
    })
  }

  #[allow(dead_code)]
  pub fn clear_all(&self) -> anyhow::Result<()>
  {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("clearing cache");
    std::fs::remove_dir_all(&self.path)?;
    std::fs::create_dir_all(&self.path)?;
    Ok(())
  }

  pub fn put(&self, tarball_path: &str) -> anyhow::Result<()>
  {
    let path = PathBuf::from(tarball_path);
    std::fs::copy(tarball_path, &self.path.join(
      path.file_name().unwrap().to_str().unwrap()
    ))?;
    Ok(())
  }
}

impl PackageGet for Cache
{
  #[tokio::main]
  async fn get(&self, dependency: &Dependency, allow_sources: bool) -> anyhow::Result<PathBuf>
  {
    let dep = self.latest_satisfied(dependency, allow_sources)?;

    for x in std::fs::read_dir(&self.path)? {
      let path = x?.path();
      let found = Dependency::from_package_name(path.file_name().unwrap().to_str().unwrap())?;
      if found == dep { return Ok(path); }
    }
    Err(anyhow!("no such package in cache: {}", dep))
  }

  fn latest_satisfied(&self, dependency: &Dependency, allow_sources: bool) -> anyhow::Result<Dependency>
  {
    let pb = ProgressBar::new_spinner()
      .with_message(format!("searching for {}", dependency))
      .with_finish(ProgressFinish::AndClear);
    let mut valid_versions = Vec::new();
    for x in std::fs::read_dir(&self.path)? {
      let path = x?.path();
      let d = Dependency::from_package_name(path.file_name().unwrap().to_str().unwrap())?;
      if d.ranged_compare(dependency) {
        valid_versions.push(d);
      }
    }
    let mut found = valid_versions
      .iter()
      .cloned()
      .max_by(|x, y| x.version.cmp(&y.version));
    if found.is_none() && !allow_sources {
      bail!("no such package in cache: {}", dependency)
    }

    let mut is_source = false;
    if found.is_none() && allow_sources {
      let src = dependency.as_sources_dependency();
      let mut valid_versions = Vec::new();
      for x in std::fs::read_dir(&self.path)? {
        let path = x?.path();
        let found = Dependency::from_package_name(path.file_name().unwrap().to_str().unwrap())?;
        if found.ranged_compare(&src) {
          valid_versions.push(found);
        }
      }
      found = valid_versions
        .iter()
        .cloned()
        .max_by(|x, y| x.version.cmp(&y.version));
      is_source = true;
      if found.is_none() {
        bail!("no such package in cache: {}", dependency)
      }
    }
    pb.finish_with_message(format!("{:<70} (latest: {})",
      format!("found {} package {}@{}/{}/{}/{} in {}",
        if is_source { String::from("source").magenta().bold() } else { String::from("pre-built").green().bold() },
        dependency.name.cyan(),
        dependency.version.to_string().bold().green(),
        dependency.arch.to_string().bold().dimmed(),
        dependency.os.to_string().bold().dimmed(),
        dependency.distribution.to_string().bold().blue().dimmed(),
        String::from("cache").bold().yellow()
      ),
      found.as_ref().context("something went wrong")?.version.to_string().green().bold()
    ));
    Ok(found.context("something went wrong")?)
  }
}