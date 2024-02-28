use std::collections::HashMap;
use colored::Colorize;
use log::{debug, info};
use crate::consts::POPPY_MANIFEST_NAME;
use crate::utils::helper_types::{Distribution, Version};

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct Manifest
{
  pub package: ManifestPackage,
  pub dependencies: Option<HashMap<String, ManifestDependencyData>>
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ManifestPackage
{
  pub name: String,
  pub version: Version,
  pub authors: Option<Vec<String>>,
  pub description: Option<String>
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct ManifestDependencyData
{
  pub version: Version,
  pub distribution: Distribution
}

impl Default for ManifestPackage
{
  fn default() -> Self
  {
    Self {
      name: String::new(),
      version: Version::default(),
      authors: None,
      description: None
    }
  }
}

impl Manifest {
  pub fn from_cli_input() -> anyhow::Result<Self>
  {
    info!("please, enter package name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    info!("please, enter package version:");
    let mut version = String::new();
    std::io::stdin().read_line(&mut version)?;
    let version = Version::try_from(version.trim().to_string())?;
    info!("please, enter authors of package (separated by comma):");
    info!("note: you can leave it blank if you don't want to specify it");
    let mut authors = String::new();
    std::io::stdin().read_line(&mut authors)?;
    let authors = if authors.is_empty() {
      None
    } else {
      Some(authors.split(",").map(|s| s.trim().to_string()).collect())
    };
    info!("please, enter description of package:");
    info!("note: you can leave it blank if you don't want to specify it");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description)?;
    let description = if description.is_empty() {
      None
    } else {
      Some(description.trim().to_string())
    };
    Ok(Self {
      package: ManifestPackage {
        name: name.trim().to_string(),
        version,
        authors,
        description
      },
      dependencies: None
    })
  }

  pub fn save(&self) -> anyhow::Result<()>
  {
    let path = std::env::current_dir()?.join(POPPY_MANIFEST_NAME);
    if path.exists() {
      anyhow::bail!("manifest already exists");
    }
    std::fs::write(&path, toml::to_string(&self)?)?;
    debug!("manifest saved to {}", path.to_str().unwrap().dimmed());
    Ok(())
  }
}