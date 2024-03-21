use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::manifest::dependency::ManifestDependencyData;
use crate::types::{Distribution, VersionRange};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest
{
  pub this: ManifestSectionThis,
  pub needs: Option<HashMap<String, ManifestDependencyData>>,
  pub build: Option<HashMap<String, VersionRange>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSectionThis
{
  pub name: String,
  pub version: VersionRange,
  pub description: Option<String>,
  pub authors: Option<Vec<String>>,
  pub license: Option<String>
}

// tests here
