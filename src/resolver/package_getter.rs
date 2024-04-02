use std::path::PathBuf;
use crate::resolver::Dependency;

pub trait PackageGet
{
  fn get(&self, dependency: &Dependency, allow_sources: bool) -> anyhow::Result<PathBuf>;
  fn latest_satisfied(&self, dependency: &Dependency, allow_sources: bool) -> anyhow::Result<Dependency>;
}