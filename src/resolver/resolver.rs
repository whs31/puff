use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;
use anyhow::{anyhow, bail, Context};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressFinish};
use crate::artifactory::Registry;
use crate::core;
use crate::manifest::Manifest;
use crate::resolver::{Dependency, dependency, PackageGet, ResolverEntry};

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

    for x in tree {
      println!("=> {:<70}   [{:<10}]   ({})",
               x.dependency.pretty_print(),
               if !x.require_build { "pre-built" } else { "sources" },
               x.tar_path.to_str().unwrap().to_string().dimmed()
      );
    }
    Ok(())
  }

  pub fn collect_recursively(&self, manifest: Manifest) -> anyhow::Result<Vec<ResolverEntry>>
  {
    if manifest.needs.is_none() || manifest.needs.as_ref().unwrap().is_empty() {
      return Ok(Vec::new());
    }

    let mut deps: Vec<ResolverEntry> = Vec::new();
    for x in manifest.needs.as_ref().unwrap() {
      let dependency = Dependency::new(
        x.0.to_string(),
        x.1.version,
        self.env.arch,
        self.env.os,
        x.1.distribution
      );

      let entry = self.try_get(&dependency)?;
      let folded_manifest = Manifest::from_tar_gz(entry.tar_path.to_str().context("failed to convert path to string")?)?;
      let sub_deps = self.collect_recursively(folded_manifest)?;
      deps.extend(sub_deps);
      deps.push(entry);
    }

    Ok(deps)
  }

  pub fn try_get(&self, dependency: &Dependency) -> anyhow::Result<ResolverEntry>
  {
    let pb = ProgressBar::new_spinner().with_finish(ProgressFinish::AndLeave);
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(format!("searching for {}", dependency.pretty_print()));
    match self.cache.get(&dependency, false) {
      Ok(x) => Ok(ResolverEntry::new(dependency.with_updated_version_from_archive_name(x.as_path())?, false, x)),
      Err(_) => match self.registry.borrow().get(&dependency, false) {
        Ok(x) => Ok(ResolverEntry::new(dependency.with_updated_version_from_archive_name(x.as_path())?, false, x)),
        Err(_) => match self.cache.get(&dependency, true) {
          Ok(x) => Ok(ResolverEntry::new(dependency.with_updated_version_from_archive_name(x.as_path())?, true, x)),
          Err(_) => match self.registry.borrow().get(&dependency, true) {
            Ok(x) => Ok(ResolverEntry::new(dependency.with_updated_version_from_archive_name(x.as_path())?, true, x)),
            Err(e) => Err(anyhow!("failed to get package: {}", e))
          },
        },
      },
    }
  }
}