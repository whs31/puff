use std::cell::RefCell;
use std::env::temp_dir;
use std::path::{Path};
use std::rc::Rc;
use std::time::Duration;
use anyhow::{anyhow, Context};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use crate::artifactory::Registry;
use crate::builder::Recipe;
use crate::core;
use crate::manifest::Manifest;
use crate::names::{DEPENDENCIES_FOLDER};
use crate::resolver::{Dependency, PackageGet, ResolverEntry};
use crate::toolchains::{CMakeToolchain, ShellToolchain, Toolchain};
use crate::types::Distribution;

pub struct Resolver
{
  pub config: Rc<core::Config>,
  pub env: Rc<core::Environment>,
  pub registry: Rc<RefCell<Registry>>,
  pub cache: Rc<crate::cache::Cache>,
  source_only: bool
}

impl Resolver
{
  pub fn new(config: Rc<core::Config>, env: Rc<core::Environment>, registry: Rc<RefCell<Registry>>, cache: Rc<crate::cache::Cache>, source_only: bool) -> Self
  {
    Self
    {
      config,
      env,
      registry,
      cache,
      source_only
    }
  }

  pub fn resolve(&self, path: &str) -> anyhow::Result<()>
  {
    let manifest = Manifest::from_directory(path)?;
    println!("\n{} {}",
      "resolving dependency tree for".to_string().bold().cyan(),
      manifest.this.name.bold().magenta()
    );

    let tree = self.collect_recursively(manifest.clone())?;
    let mut tree = tree
      .into_iter()
      .fold(Vec::new(), |mut acc, x| {
        if !acc.iter().any(|y: &ResolverEntry| y.dependency == x.dependency) { acc.push(x); }
        acc
      });

    for x in &mut tree {
      if x.require_build {
        self.build(x)?;
      }
    }

    let pb = ProgressBar::new(tree.len() as u64);
    pb.set_message("installing dependencies");
    pb.set_style(
      ProgressStyle::with_template("{spinner:.green} {wide_msg} [{elapsed}] [{bar:30.yellow/yellow}] {human_pos:4}/{human_len:4} ({percent:3})")
        .unwrap()
        .progress_chars("█▒░")
    );
    let install_path = Path::new(path)
      .join(DEPENDENCIES_FOLDER);
    for x in &tree {
      pb.set_message(format!("installing {}", x.dependency.pretty_print()));
      x.install(install_path.to_str().context("failed to convert path to string")?)?;
      pb.inc(1);
      print!("\x1b[A\x1b[2K\r");
      println!("✅ installed {}\n", x.dependency.pretty_print());
    }
    println!("\x1b[A\x1b[2K\r");
    pb.finish_with_message(format!("installed {} dependencies for {}",
      tree.len().to_string().magenta().bold(),
      manifest.this.name.bold().green()
    ));
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
    if self.source_only && !dependency.name.starts_with("cmake") {
      match self.cache.get(&dependency, false) {
        Ok(x) => Ok(ResolverEntry::new(dependency.with_updated_version_from_archive_name(x.as_path())?, false, x)),
          Err(_) => match self.cache.get(&dependency, true) {
            Ok(x) => Ok(ResolverEntry::new(dependency.with_updated_version_from_archive_name(x.as_path())?, true, x)),
            Err(_) => match self.registry.borrow().get(&dependency, true) {
              Ok(x) => Ok(ResolverEntry::new(dependency.with_updated_version_from_archive_name(x.as_path())?, true, x)),
              Err(e) => Err(anyhow!("failed to get package: {}", e))
            },
          },
      }
    } else {
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

  pub fn build(&self, entry: &mut ResolverEntry) -> anyhow::Result<ResolverEntry>
  {
    let pb = ProgressBar::new_spinner()
      .with_finish(ProgressFinish::AndClear);
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(format!("building {}", entry.dependency.pretty_print()));
    let build_directory = temp_dir()
      .join(entry.dependency.name.clone())
      .join(entry.dependency.version.to_string().clone());
    std::fs::create_dir_all(&build_directory)?;
    crate::pack::unpack(entry.tar_path.to_str().unwrap(), build_directory.to_str().unwrap())?;
    let _manifest = Manifest::from_directory(build_directory.to_str().unwrap())?;
    let recipe = Recipe::from_directory(build_directory.to_str().unwrap())?;

    self.resolve(build_directory.to_str().unwrap())?;

    let recipe_toolchain = match entry.dependency.distribution {
      Distribution::Static => recipe.static_toolchain.clone().context(format!("recipe for {} does not have a static or shared toolchain", entry.dependency))?,
      Distribution::Shared => recipe.shared_toolchain.clone().context(format!("recipe for {} does not have a static or shared toolchain", entry.dependency))?,
      _ => { return Err(anyhow!("unsupported distribution for build: {} (package {})", entry.dependency.distribution, entry.dependency)); }
    };

    let export_dir = if recipe_toolchain.toolchain.cmake.is_some() {
      CMakeToolchain::new(&self.config)
        .build_from_recipe(&recipe, build_directory.to_str().unwrap(), entry.dependency.distribution.clone())?
    } else if recipe_toolchain.toolchain.shell.is_some() {
      ShellToolchain::new()
        .build_from_recipe(&recipe, build_directory.to_str().unwrap(), entry.dependency.distribution.clone())?
    } else {
      return Err(anyhow!("unsupported toolchain for build: {:?}", recipe_toolchain.toolchain));
    };

    let tarball = crate::pack::pack_for_cache(
      export_dir.to_str().unwrap(),
      entry.dependency.arch.clone(),
      entry.dependency.distribution.clone(),
      entry.dependency.os.clone()
    )?;
    self.cache.put(tarball.as_str())?;
    entry.tar_path = self.cache.get(&entry.dependency, false)?;

    Ok(ResolverEntry::new(entry.dependency.clone(), true, build_directory))
  }
}