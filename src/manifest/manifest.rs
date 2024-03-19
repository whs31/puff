use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest
{
  pub this: ManifestSectionThis,
  pub needs: Option<HashMap<String, ManifestDependencyData>>,
  pub build: Option<HashMap<String, Version>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestSectionThis
{
  pub name: String,
  pub version: Version,
  pub description: Option<String>,
  pub authors: Option<Vec<String>>,
  pub license: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestDependencyData // todo: deserialize it from either string or struct
{
  pub version: Version,
  pub distribution: Option<Distribution>
}

// tests here
