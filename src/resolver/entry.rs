use std::path::PathBuf;
use crate::resolver::Dependency;

#[derive(Debug, Clone)]
pub struct ResolverEntry
{
  pub dependency: Dependency,
  pub require_build: bool,
  pub tar_path: PathBuf
}

impl ResolverEntry
{
  pub fn new(dependency: Dependency, require_build: bool, tar_path: PathBuf) -> Self
  {
    Self
    {
      dependency,
      require_build,
      tar_path
    }
  }
}