use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use colored::Colorize;
use log::{debug, info, trace, warn};
use walkdir::WalkDir;
use crate::args::Args;
use crate::artifactory::Artifactory;
use crate::registry;
use crate::registry::entry::{RegistryEntry, RegistryEntryRaw};
use crate::resolver::Dependency;
use crate::utils::Config;

pub struct Registry
{
  pub packages: Vec<RegistryEntry>,
  config: Rc<RefCell<Config>>,
  artifactory: Rc<Artifactory>,
  registry_path: String,
  args: Rc<Args>
}

impl Registry
{
  pub fn new(config: Rc<RefCell<Config>>, artifactory: Rc<Artifactory>, path: &str, args: Rc<Args>) -> Self
  {
    Self
    {
      packages: vec![],
      config,
      artifactory,
      registry_path: String::from(path),
      args
    }
  }

  pub fn sync_aql(&mut self, lazy: bool) -> anyhow::Result<&mut Self>
  {
    info!("syncing with remote repository {}", "via aql".green().bold());
    debug!("syncing into cache ({})", &self.registry_path.dimmed());
    std::fs::create_dir_all(Path::new(&self.registry_path).parent().unwrap())?;

    if lazy {
      warn!("lazy sync is enabled. updating remote registry will not be performed unless cached registry is broken.");
      if Path::new(&self.registry_path).exists() {
        warn!("older registry found. skipping aql sync");
        // todo: cache registry
        //return Ok(self);
      }
    }

    let raw = self.artifactory.query(r#"items.find({"repo": "poppy-cxx-repo", "name": {"$match": "*"}}).sort({"$desc": ["created"]})"#)?;
    let parsed: crate::artifactory::query::PackageQueryResponse = serde_json::from_str(&raw)?;

    for entry in parsed.results
    {
      let y = Dependency::from_package_name(entry.name.as_str())?;
      // trace!("found package: {}/{}/{}@{}", y.name, y.arch, y.distribution, y.version);
      if self.packages
        .iter()
        .any(|x| x.name == y.name)
      {
        let reg_entry = self.packages
          .iter_mut()
          .find(|x| x.name == y.name)
          .context("weird things happened...")?;
        if reg_entry.versions
          .contains_key(&y.version)
        {
          if reg_entry.versions
            .get_mut(&y.version)
            .unwrap()
            .contains_key(&y.distribution)
          {
            if reg_entry.versions
              .get_mut(&y.version)
              .unwrap()
              .get_mut(&y.distribution)
              .unwrap()
              .contains(&y.arch)
            {
              trace!("duplicate package: {}/{}/{}@{}", y.name, y.arch, y.distribution, y.version);
              continue;
            }

            reg_entry.versions
              .get_mut(&y.version)
              .unwrap()
              .get_mut(&y.distribution)
              .unwrap()
              .push(y.arch);
          } else {
            reg_entry.versions
              .get_mut(&y.version)
              .unwrap()
              .insert(y.distribution, vec![y.arch]);
          }
        } else {
          reg_entry.versions
            .insert(y.version, HashMap::from([(y.distribution, vec![y.arch])]));
        }
      } else {
        let reg_entry = RegistryEntry {
          name: y.name,
          versions: HashMap::from([(y.version, HashMap::from([(y.distribution, vec![y.arch])]))]),
        };
        self.packages.push(reg_entry);
      }
    }

    for entry in &self.packages
    {
      debug!("found package: {}", &entry.pretty_format());
    }

    info!("sync done (found {} packages)", self.packages.len());

    Ok(self)
  }

  pub fn contains(&self, dependency: &Dependency) -> bool
  {
    self.packages.iter().any(|x| {
      x.name == dependency.name
        && x.versions.contains_key(&dependency.version)
        && x.versions[&dependency.version].contains_key(&dependency.distribution)
        && x.versions[&dependency.version][&dependency.distribution].contains(&dependency.arch)
    })
  }
}
