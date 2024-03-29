use std::fmt::Display;
use std::str::FromStr;
use anyhow::Context;
use colored::Colorize;
use crate::types::{Arch, Distribution, OperatingSystem, VersionRange};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dependency
{
  pub name: String,
  pub version: VersionRange,
  pub arch: Arch,
  pub os: OperatingSystem,
  pub distribution: Distribution
}

impl Display for Dependency
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}@{}/{}/{}/{}", self.name, self.version, self.arch, self.os, self.distribution)
  }
}

impl Dependency
{
  pub fn new(name: String, version: VersionRange, arch: Arch, os: OperatingSystem, distribution: Distribution) -> Self
  {
    Self
    {
      name,
      version,
      arch,
      os,
      distribution
    }
  }

  pub fn from_package_name(package_name: &str) -> anyhow::Result<Self>
  {
    let re_name = regex::Regex::new(r"^([a-zA-Z0-9_+]+)")?;
    let captures_name = re_name
      .captures(package_name)
      .context("invalid package name")?;
    let re_version = regex::Regex::new(r"-([0-9]+\.[0-9]+\.[0-9]+)-")?;
    let captures_version = re_version
      .captures(package_name)
      .context("invalid package name")?;
    let re_arch = regex::Regex::new(r"(-([a-zA-Z0-9_]+)-([a-zA-Z0-9]+))")?;
    let captures_arch = re_arch
      .captures(package_name)
      .context("invalid package name")?;
    let re_distribution = regex::Regex::new(r"-([a-zA-Z0-9]+)\.tar")?;
    let captures_distribution = re_distribution
      .captures(package_name)
      .context("invalid package name")?;

    Ok(Self
    {
      name: captures_name.get(1).unwrap().as_str().to_string(),
      version: VersionRange::from_str(captures_version.get(1).unwrap().as_str())?,
      arch: Arch::from_str(captures_arch.get(2).unwrap().as_str())?,
      os: OperatingSystem::from_str(captures_arch.get(3).unwrap().as_str())?,
      distribution: Distribution::from_str(captures_distribution.get(1).unwrap().as_str())?,
    })
  }

  pub fn from_std_path(path: &std::path::Path) -> anyhow::Result<Self>
  {
    Self::from_package_name(path.file_name().unwrap().to_str().unwrap())
  }

  pub fn with_updated_version(&self, rhs: &Dependency) -> anyhow::Result<Self>
  {
    Ok(Self {
      name: self.name.clone(),
      version: rhs.version.clone(),
      arch: self.arch.clone(),
      os: self.os.clone(),
      distribution: self.distribution.clone(),
    })
  }

  pub fn with_updated_version_from_archive_name(&self, path: &std::path::Path) -> anyhow::Result<Self>
  {
    let dep = Self::from_std_path(path)?;
    self.with_updated_version(&dep)
  }

  pub fn ranged_compare(&self, other: &Dependency) -> bool
  {
    self.name == other.name
      && self.version.min >= other.version.min
      && self.version.max <= other.version.max
      && self.arch == other.arch
      && self.os == other.os
  }

  pub fn as_sources_dependency(&self) -> Dependency
  {
    let mut dep = self.clone();
    dep.distribution = Distribution::Sources;
    dep.arch = Arch::Unknown;
    dep.os = OperatingSystem::Unknown;
    dep
  }

  pub fn pretty_print(&self) -> String
  {
    format!("{}@{}/{}/{}/{}",
            self.name.bold().yellow(),
            self.version.to_string().bold().blue(),
            self.arch.to_string().dimmed(),
            self.os.to_string().dimmed(),
            self.distribution.to_string().bold()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_package_name() {
    let dep = Dependency::from_package_name("cmake-0.1.3-unknown-unknown-sources.tar.gz");
    assert!(dep.is_ok());
    assert_eq!(dep.as_ref().unwrap().clone().name, "cmake");
    assert_eq!(dep.as_ref().unwrap().clone().version, VersionRange::from_str("0.1.3").unwrap());
    assert_eq!(dep.as_ref().unwrap().clone().arch, Arch::Unknown);
    assert_eq!(dep.as_ref().unwrap().clone().os, OperatingSystem::Unknown);
    assert_eq!(dep.as_ref().unwrap().clone().distribution, Distribution::Sources);

    let dep = Dependency::from_package_name("fmt-1.1.3-x86_64-windows-static.tar.gz");
    assert!(dep.is_ok());
    assert_eq!(dep.as_ref().unwrap().clone().name, "fmt");
    assert_eq!(dep.as_ref().unwrap().clone().version, VersionRange::from_str("1.1.3").unwrap());
    assert_eq!(dep.as_ref().unwrap().clone().arch, Arch::X86_64);
    assert_eq!(dep.as_ref().unwrap().clone().os, OperatingSystem::Windows);
    assert_eq!(dep.as_ref().unwrap().clone().distribution, Distribution::Static);
  }
}