use crate::utils::helper_types::{Distribution, PlatformArch, Version};

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
}