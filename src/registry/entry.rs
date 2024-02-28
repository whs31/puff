use std::collections::HashMap;
use crate::utils::helper_types::{Distribution, PlatformArch, Version};

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct RegistryEntryRaw
{
  pub name: String,
  pub versions: HashMap<String, RegistryEntryRawVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct RegistryEntryRawVersion
{
  pub distributions: HashMap<String, Vec<String>>,
}

pub struct RegistryEntry
{
  pub name: String,
  pub versions: HashMap<Version, HashMap<Distribution, Vec<PlatformArch>>>
}

impl TryFrom<RegistryEntryRaw> for RegistryEntry
{
  type Error = anyhow::Error;

  fn try_from(value: RegistryEntryRaw) -> Result<Self, Self::Error>
  {
    let mut versions = HashMap::new();
    for (version, raw_version) in value.versions
    {
      let mut distributions = HashMap::new();
      for (distribution, platforms) in raw_version.distributions
      {
        let platforms = platforms
          .into_iter()
          .map(|platform| PlatformArch::from(platform.as_str()))
          .collect::<Vec<PlatformArch>>();
        distributions.insert(Distribution::from(distribution.as_str()), platforms);
      }
      versions.insert(
        Version::try_from(version.as_str())?,
        distributions
      );
    }
    Ok(Self {
      name: value.name,
      versions
    })
  }
}