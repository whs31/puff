use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;
use anyhow::bail;
use indicatif::ProgressBar;
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
    for x in std::fs::read_dir(&self.path)? {
      let path = x?.path();
      let found = Dependency::from_package_name(path.file_name().unwrap().to_str().unwrap())?;
      if found.ranged_compare(dependency) {
        return Ok(path);
      }
    }

    if !allow_sources {
      bail!("no such package in cache: {}", dependency)
    }

    for x in std::fs::read_dir(&self.path)? {
      let path = x?.path();
      let found = Dependency::from_package_name(path.file_name().unwrap().to_str().unwrap())?;
      let dependency = dependency.as_sources_dependency();
      if found.ranged_compare(&dependency) {
        return Ok(path);
      }
    }

    bail!("no such package in cache: {}", dependency)
  }

  fn latest_satisfied(&self, dependency: &Dependency, allow_sources: bool) -> anyhow::Result<Dependency>
  {
    todo!()
  }
}