use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe
{
  #[serde(rename = "static")]
  pub static_toolchain: Option<ToolchainSection>,

  #[serde(rename = "shared")]
  pub shared_toolchain: Option<ToolchainSection>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ToolchainSection
{
  pub cmake: Option<CMakeSection>,
  pub shell: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CMakeSection
{
  pub generator: Option<String>,
  pub definitions: Option<HashMap<String, String>>,
}