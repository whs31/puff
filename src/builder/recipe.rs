use std::collections::HashMap;
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use crate::names::{EXTENSIONS_DIRECTORY, RECIPE_FILE};
use crate::types::Distribution;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipe
{
  #[serde(rename = "static")]
  pub static_toolchain: Option<Toolchain>,

  #[serde(rename = "shared")]
  pub shared_toolchain: Option<Toolchain>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Toolchain
{
  pub toolchain: ToolchainSection
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolchainSection
{
  pub cmake: Option<CMakeSection>,
  pub shell: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CMakeSection
{
  pub generator: Option<String>,
  pub definitions: Option<HashMap<String, String>>,
}

impl Recipe
{
  pub fn from_directory(root_path: &str) -> anyhow::Result<Self>
  {
    if !std::path::Path::new(root_path).join(EXTENSIONS_DIRECTORY).join(RECIPE_FILE).exists() {
      return Err(anyhow::anyhow!("recipe file not found in path ({})", root_path))
    }
    Ok(serde_yaml::from_str(&std::fs::read_to_string(std::path::Path::new(root_path)
      .join(EXTENSIONS_DIRECTORY)
      .join(RECIPE_FILE)
      .as_path()
    )?)?)
  }

  pub fn extract_toolchain(&self, distribution: Distribution) -> anyhow::Result<ToolchainSection>
  {
    match distribution {
      Distribution::Static => Ok(self
        .static_toolchain
        .as_ref()
        .context("toolchain was requested to build as static but no static toolchain was provided")?
        .toolchain
        .clone()),
      Distribution::Shared => Ok(self
        .shared_toolchain
        .as_ref()
        .context("toolchain was requested to build as shared but no shared toolchain was provided")?
        .toolchain
        .clone()),
      _ => Err(anyhow!("unsupported distribution for build: {}", distribution))
    }
  }
}