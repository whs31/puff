use std::rc::Rc;
use crate::core;

pub struct Resolver
{
  pub config: Rc<core::Config>,
  pub env: Rc<core::Environment>,
  pub registry: Rc<crate::artifactory::Registry>,
  pub cache: Rc<crate::cache::Cache>
}

impl Resolver
{
  pub fn new(config: Rc<core::Config>, env: Rc<core::Environment>, registry: Rc<crate::artifactory::Registry>, cache: Rc<crate::cache::Cache>) -> Self
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
    Ok(())
  }
}