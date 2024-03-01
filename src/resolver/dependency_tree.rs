use anyhow::Context;
use colored::Colorize;
use log::{debug, info, trace};
use crate::manifest::Manifest;
use crate::registry::Registry;
use crate::resolver::{Cache, Dependency};
use crate::utils::helper_types::PlatformArch;

pub struct DependencyStack
{
  pub cache: Cache,
  stack: Vec<Dependency>
}

impl DependencyStack
{
  pub fn new(cache_path: &str) -> anyhow::Result<Self>
  {
    Ok(Self
    {
      cache: Cache::new(cache_path)?,
      stack: Vec::new()
    })
  }

  // todo: maybe push manifest as whole?
  // todo: also maybe hide push/pop from user?

  pub fn resolve(&mut self, manifest: &Manifest, reg: &Registry, arch: PlatformArch) -> anyhow::Result<&mut Self>
  {
    info!("resolving dependencies for top-level package {} for arch {}",
      &manifest.package.name.yellow(),
      &arch.to_string().yellow()
    );
    let dep = self.resolve_recursively(manifest, reg, arch)?;
    self.stack = dep;
    Ok(self)
  }

  fn resolve_recursively(&self, manifest: &Manifest, reg: &Registry, arch: PlatformArch) -> anyhow::Result<Vec<Dependency>>
  {
    debug!("resolving dependencies for package {}", &manifest.package.name.magenta());
    if manifest.dependencies.is_none() || manifest.dependencies.as_ref().unwrap().is_empty() {
      debug!("{} has no direct dependencies!", &manifest.package.name.magenta());
      return Ok(Vec::new())
    }

    let deps = manifest.dependencies
      .as_ref()
      .context("failed conversion from hashmap to vec")?
      .iter()
      .map(|dep| {
        Dependency::new(dep.0.to_string(), dep.1.version.clone(), dep.1.distribution.clone(), arch)
      })
      .collect::<Vec<Dependency>>();
    for dep in deps {
      trace!("resolving dependency {}", &dep.name.yellow());
    }
    debug!("resolving package {} - done!", &manifest.package.name.magenta());
    Ok(Vec::new()) // todo
  }

  pub fn push(&mut self, dependency: Dependency) -> anyhow::Result<&mut Self>
  {
    // if self.check(&dependency) todo
    self.stack.push(dependency);
    Ok(self)
  }

  pub fn pop(&mut self) -> Option<Dependency> { self.stack.pop() }
  pub fn len(&self) -> usize { self.stack.len() }
  pub fn is_empty(&self) -> bool  { self.stack.is_empty() }

  pub fn check(&self, dependency: &Dependency) -> bool
  {
    todo!()
  }
}