use std::path::PathBuf;
use std::rc::Rc;
use anyhow::ensure;
use colored::Colorize;
use indicatif::ProgressBar;
use crate::resolver::PackageGet;

pub struct Registry
{
  pub remotes: Vec<crate::artifactory::Artifactory>,

  #[allow(dead_code)]
  config: Rc<crate::core::Config>
}

impl Registry
{
  pub fn new(config: Rc<crate::core::Config>) -> anyhow::Result<Self>
  {
    let mut remotes = Vec::new();
    for x in &config.registry.list
    {
      let artifactory = crate::artifactory::Artifactory::new(config.clone(), &x.name)?;
      remotes.push(artifactory);
    }
    Ok(Self
    {
      remotes,
      config
    })
  }

  pub fn ping_all(&self) -> anyhow::Result<&Self>
  {
    for x in &self.remotes {
      x.ping()?;
    }
    Ok(self)
  }

  pub fn sync_all(&mut self) -> anyhow::Result<&Self>
  {
    let pb = ProgressBar::new(self.remotes.len() as u64);
    pb.set_style(
      indicatif::ProgressStyle::with_template("{elapsed} {spinner:.green} [{bar:30.white/white}] {msg} ({pos}/{len})")
        .unwrap()
    );
    pb.set_message("syncing remotes");
    for x in &mut self.remotes {
      x.sync_aql()?;
      pb.inc(1);
    }
    pb.finish_and_clear();
    println!("found {} packages in {} remotes",
       self.remotes.iter().map(|x| x.available_packages.len()).sum::<usize>().to_string().bold().green(),
       self.remotes.len().to_string().bold().magenta()
    );
    Ok(self)
  }
}

impl PackageGet for Registry
{
  fn get(&self, dependency: &crate::resolver::Dependency, allow_sources: bool) -> anyhow::Result<PathBuf>
  {
    let mut result = PathBuf::new();
    let mut error: String = String::new();
    for x in &self.remotes {
      match x.get(dependency, allow_sources) {
        Ok(x) => {
          result = x;
          break;
        }
        Err(x) => error = x.to_string()
      }
    }
    ensure!(result.is_file(), "{}", error);
    Ok(result)
  }
}