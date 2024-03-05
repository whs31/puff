use anyhow::Context;
use crate::utils::helper_types::{Distribution, PlatformArch, Version};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Dependency
{
  pub name: String,
  pub version: Version,
  pub distribution: Distribution,
  pub arch: PlatformArch
}

impl Dependency
{
  pub fn new(name: String, version: Version, distribution: Distribution, arch: PlatformArch) -> Self
  {
    Self
    {
      name,
      version,
      distribution,
      arch
    }
  }

  pub fn from_package_name(package_name: &str) -> anyhow::Result<Self>
  {
    // poppy-1.0.10-linux-x64-executable.tar.gz
    // name-versionmajor.minor.patch-arch-distribution.tar.gz

    // name: ^([a-zA-Z0-9]+)
    // version: -([0-9]+\.[0-9]+\.[0-9]+)-
    // arch: -([a-zA-Z0-9-]+)-
    // distribution: -([a-zA-Z0-9]+)\.tar

    let re_name = regex::Regex::new(r"^([a-zA-Z0-9]+)")?;
    let captures_name = re_name
      .captures(package_name)
      .context("invalid package name")?;
    let re_version = regex::Regex::new(r"-([0-9]+\.[0-9]+\.[0-9]+)-")?;
    let captures_version = re_version
      .captures(package_name)
      .context("invalid package name")?;
    let re_arch = regex::Regex::new(r"-([a-zA-Z0-9-]+)-")?;
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
      version: Version::try_from(captures_version.get(1).unwrap().as_str())?,
      distribution: Distribution::from(captures_distribution.get(1).unwrap().as_str()),
      arch: PlatformArch::from(captures_arch.get(1).unwrap().as_str())
    })
  }
}