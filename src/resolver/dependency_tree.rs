use std::path::MAIN_SEPARATOR;
use anyhow::Context;
use colored::Colorize;
use log::{debug, error, info, trace, warn};
use crate::manifest::Manifest;
use crate::registry::Registry;
use crate::resolver::{Cache, Dependency};
use crate::utils::helper_types::{Distribution, PlatformArch};

pub struct DependencyStack
{
  pub cache: Cache,
  stack: Vec<DependencyStackItem>
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct DependencyStackItem
{
  pub(in self) dependency: Dependency,
  pub(in self) archive_path: String
}

impl DependencyStackItem {
  pub fn pretty_print(&self) -> String
  {
    format!("{}@{}/{}/{} => .../{}",
      self.dependency.name.magenta(),
      self.dependency.version.to_string().green(),
      self.dependency.distribution.to_string().cyan(),
      self.dependency.arch.to_string().blue(),
      self.archive_path[self.archive_path
        .rfind(MAIN_SEPARATOR)
        .context("failed to find separator"
        ).expect("this should never happen") + 1..]
        .white()
        .dimmed()
    )
  }
}

impl DependencyStack
{
  pub fn new(cache_path: &str, artifactory_url: &str, artifactory_api_url: &str, oauth: (&str, &str)) -> anyhow::Result<Self>
  {
    Ok(Self
    {
      cache: Cache::new(cache_path, artifactory_url, artifactory_api_url, oauth)?,
      stack: Vec::new()
    })
  }

  pub fn resolve(&mut self, manifest: &Manifest, reg: &Registry, arch: PlatformArch) -> anyhow::Result<&mut Self>
  {
    info!("resolving dependencies for top-level package {} for arch {}",
      &manifest.package.name.yellow(),
      &arch.to_string().yellow()
    );
    let manifest = Manifest::from_pwd()?;
    let raw = self.resolve_recursively(manifest, reg, arch)?;
    info!("found {} direct and indirect dependencies", raw.len());
    raw
      .iter()
      .for_each(|d| trace!("- {}", d.pretty_print()));
    // remove duplicates
    let mut seen = std::collections::HashSet::new();
    self.stack = raw
      .into_iter()
      .filter(|dep| seen.insert(dep.dependency.clone()))
      .collect::<Vec<DependencyStackItem>>();
    info!("resolved {} dependencies", self.stack.len());
    self.stack
      .iter()
      .for_each(|d| info!("- {}", d.pretty_print()));
    Ok(self)
  }

  fn resolve_recursively(&self, manifest: Manifest, reg: &Registry, arch: PlatformArch) -> anyhow::Result<Vec<DependencyStackItem>>
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
        let arch_or_any = match &dep.1.distribution {
          Distribution::Sources => PlatformArch::Any,
          _ => arch
        };
        Dependency::new(dep.0.to_string(), dep.1.version.clone(), dep.1.distribution.clone(), arch_or_any)
      })
      .collect::<Vec<Dependency>>();
    let mut res = Vec::new();
    for dep in deps {
      let name_f = format!("{}@{}/{}/{}",
        &dep.name.yellow(),
        &dep.version.to_string().purple(),
        &dep.distribution.to_string().purple(),
        &dep.arch.to_string().blue()
      );
      trace!("resolving direct dependency {}", &dep.name.yellow());
      if !reg.contains(&dep) {
        error!("dependency {name_f} not found in registry");
        error!("try updating local registry with poppy --sync or check manifest file");
        return Err(anyhow::anyhow!("dependency not found in registry"))
      }
      debug!("found {name_f} in registry");
      let archive = self.cache.get_or_download(&dep)?;
      res.push(DependencyStackItem {
        dependency: dep,
        archive_path: archive
          .to_str()
          .context("failed to convert path to string")?
          .to_string()
      });
      res.extend(
        self.resolve_recursively(
          Manifest::from_tar_gz(
            archive
              .to_str()
              .context("failed to convert path to string")?
          )?,
          reg,
          arch
        )?
      );
    }
    debug!("resolving package {} - done!", &manifest.package.name.magenta());
    Ok(res)
  }

  pub fn len(&self) -> usize { self.stack.len() }
  pub fn is_empty(&self) -> bool  { self.stack.is_empty() }
}