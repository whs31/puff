use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use colored::Colorize;
use indicatif::ProgressBar;
use crate::artifactory::Registry;
use crate::core;
use crate::manifest::Manifest;
use crate::resolver::Dependency;

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
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(format!("{} {}",
      "resolving dependency tree for".to_string().bold().cyan(),
      manifest.this.name.bold().magenta()
    ));

    let tree = self.collect_recursively(manifest)?;

    pb.finish_and_clear();
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
      // todo: registry get
    }

    Ok(Vec::new())
  }
}