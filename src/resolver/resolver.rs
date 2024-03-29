use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;
use anyhow::{anyhow, bail};
use colored::Colorize;
use indicatif::ProgressBar;
use crate::artifactory::Registry;
use crate::core;
use crate::manifest::Manifest;
use crate::resolver::{Dependency, PackageGet};

pub struct Resolver
{
  pub config: Rc<core::Config>,
  pub env: Rc<core::Environment>,
  pub registry: Rc<RefCell<Registry>>,
  pub cache: Rc<crate::cache::Cache>
}

impl Resolver
{
  pub fn new(config: Rc<core::Config>, env: Rc<core::Environment>, registry: Rc<RefCell<Registry>>, cache: Rc<crate::cache::Cache>) -> Self
  {
    Self
    {
      config,
      env,
      registry,
      cache
    }
  }

  pub fn resolve(&self, path: &str) -> anyhow::Result<()>
  {
    let manifest = Manifest::from_directory(path)?;
    println!("\n{} {}",
      "resolving dependency tree for".to_string().bold().cyan(),
      manifest.this.name.bold().magenta()
    );

    let tree = self.collect_recursively(manifest)?;
    Ok(())
  }

  pub fn collect_recursively(&self, manifest: Manifest) -> anyhow::Result<Vec<Dependency>>
  {
    if manifest.needs.is_none() || manifest.needs.as_ref().unwrap().is_empty() {
      return Ok(Vec::new());
    }

    let mut deps: Vec<Dependency> = Vec::new();
    for x in manifest.needs.as_ref().unwrap() {
      // todo: manifest: from tar gz
      // check cache for built package
      // if not, check reigstry for built package
      // if not again, check cache for source package
      // if not, check registry for source package
      // if all fails, error

      let dependency = Dependency::new(
        x.0.to_string(),
        x.1.version,
        self.env.arch,
        self.env.os,
        x.1.distribution
      );

      let tarball = self.try_get(&dependency)?;
    }

    Ok(Vec::new())
  }

  pub fn try_get(&self, dependency: &Dependency) -> anyhow::Result<PathBuf>
  {
    match self.cache.get(&dependency, false) {
      Ok(x) => Ok(x),
      Err(_) => match self.registry.borrow().get(&dependency, false) {
        Ok(x) => Ok(x),
        Err(_) => match self.cache.get(&dependency, true) {
          Ok(x) => Ok(x),
          Err(_) => match self.registry.borrow().get(&dependency, true) {
            Ok(x) => Ok(x),
            Err(e) => Err(anyhow!("failed to get package: {}", e))
          },
        },
      },
    }
  }
}