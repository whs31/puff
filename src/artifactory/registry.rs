use std::rc::Rc;

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
      artifactory.ping()?;
      remotes.push(artifactory);
    }
    Ok(Self
    {
      remotes,
      config
    })
  }
}