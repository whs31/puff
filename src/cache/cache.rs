use std::rc::Rc;
use crate::core;

pub struct Cache
{
  pub config: Rc<core::Config>,
  pub env: Rc<core::Environment>,
  pub registry: Rc<crate::artifactory::Registry>,
  pub path: String
}

impl Cache
{
  pub fn new(config: Rc<core::Config>, env: Rc<core::Environment>, registry: Rc<crate::artifactory::Registry>) -> Self
  {
    let path = config.directories.dirs.cache_dir().to_path_buf();
    Self
    {
      config,
      env,
      registry,
      path: path.to_str().unwrap().to_string()
    }
  }
}