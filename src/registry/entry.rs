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