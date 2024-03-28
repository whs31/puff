use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;
use anyhow::bail;
use indicatif::ProgressBar;
use crate::core;
use crate::resolver::{Dependency, PackageGet};
use crate::types::{Arch, Distribution, OperatingSystem};

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

  pub fn clear_all(&self) -> anyhow::Result<()>
  {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("clearing cache");
    std::fs::remove_dir_all(&self.path)?;
    std::fs::create_dir_all(&self.path)?;
    Ok(())
  }
}

impl PackageGet for Cache
{
  fn get(&self, dependency: &Dependency, allow_sources: bool) -> anyhow::Result<PathBuf>
  {
    let path = self.path
      .join(format!("{}-{}-{}-{}-{}.tar.gz",
                    dependency.name,
                    dependency.version.to_string(),
                    dependency.arch.to_string(),
                    dependency.os.to_string(),
                    dependency.distribution.to_string()
      ));
    if path.exists() {
      Ok(path)
    }
    else if allow_sources {
      let path_src = self.path
        .join(format!("{}-{}-{}-{}-{}.tar.gz",
                      dependency.name,
                      dependency.version.to_string(),
                      Arch::Unknown,
                      OperatingSystem::Unknown,
                      Distribution::Sources
        ));
      if path_src.exists() {
        Ok(path_src)
      } else {
        bail!("no such package in cache: {}", dependency)
      }
    }
    else {
      bail!("no such package in cache: {}", dependency)
    }
  }
}