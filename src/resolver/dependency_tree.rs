use std::cell::RefCell;
use std::path::{MAIN_SEPARATOR, Path};
use std::rc::Rc;
use anyhow::Context;
use colored::Colorize;
use log::{debug, error, info, trace, warn};
use crate::artifactory::Artifactory;
use crate::manifest::Manifest;
use crate::registry::Registry;
use crate::resolver::{Cache, Dependency};
use crate::utils::helper_types::{Distribution, PlatformArch};

pub struct DependencyStack
{
  pub cache: Cache,
  pub target_folder: String,
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

  pub fn install(&self, to: &str) -> anyhow::Result<()>
  {
    let target_folder = Path::new(to);
      //.join(&self.dependency.name);
    trace!("installing {} to {}", self.pretty_print(), target_folder.display());
    std::fs::create_dir_all(&target_folder)?;
    self.unpack(target_folder
      .to_str()
      .context("failed to convert path to string")?
    )
  }

  pub fn unpack(&self, to: &str) -> anyhow::Result<()>
  {
    crate::artifactory::unpack_to(self.archive_path.as_str(), to)
      .context("failed to unpack dependency archive")
  }
}

impl DependencyStack
{
  pub fn new(cache_path: &str, target_folder: &str, artifactory: Rc<Artifactory>) -> anyhow::Result<Self>
  {
    Ok(Self
    {
      cache: Cache::new(cache_path, artifactory)?,
      target_folder: String::from(target_folder),
      stack: Vec::new()
    })
  }

  pub fn resolve(&mut self, reg: Rc<RefCell<Registry>>, arch: PlatformArch) -> anyhow::Result<&mut Self>
  {
    let manifest = Manifest::from_pwd()?;
    manifest.pretty_print();
    info!("resolving dependencies for top-level package {} for arch {}",
      &manifest.package.name.yellow(),
      &arch.to_string().yellow()
    );
    let raw = self.resolve_recursively(manifest.clone(), reg, arch)?;
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

    if self.stack.is_empty() {
      warn!("no dependencies found for package {}", &manifest.package.name.magenta());
    }

    Ok(self)
  }

  fn resolve_recursively(&self, manifest: Manifest, reg: Rc<RefCell<Registry>>, arch: PlatformArch) -> anyhow::Result<Vec<DependencyStackItem>>
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
      if !reg.borrow().contains(&dep) {
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
          reg.clone(),
          arch
        )?
      );
    }
    debug!("resolving package {} - done!", &manifest.package.name.magenta());
    Ok(res)
  }

  pub fn install_dependencies(&self) -> anyhow::Result<()>
  {
    debug!("install folder: {}", &self.target_folder);
    self.stack
      .iter()
      .try_for_each(|d| d.install(self.target_folder.as_str()))
  }

  #[allow(dead_code)]
  pub fn len(&self) -> usize { self.stack.len() }

  #[allow(dead_code)]
  pub fn is_empty(&self) -> bool  { self.stack.is_empty() }
}