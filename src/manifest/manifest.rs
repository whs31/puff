use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::{Distribution, VersionRange};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest
{
  pub this: ManifestSectionThis,
  pub needs: Option<HashMap<String, ManifestDependencyData>>,
  pub build: Option<HashMap<String, VersionRange>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestSectionThis
{
  pub name: String,
  pub version: VersionRange,
  pub description: Option<String>,
  pub authors: Option<Vec<String>>,
  pub license: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestDependencyData // todo: deserialize it from either string or struct
{
  pub version: VersionRange,
  pub distribution: Option<Distribution>
}

// tests here
