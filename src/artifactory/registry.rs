use std::rc::Rc;
use std::time::Duration;
use indicatif::ProgressBar;

pub struct Registry
{
  pub remotes: Vec<crate::artifactory::Artifactory>,
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
    Ok(self)
  }
}