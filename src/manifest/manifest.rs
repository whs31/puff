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
    info!("enter package name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    info!("enter package version:");
    let mut version = String::new();
    std::io::stdin().read_line(&mut version)?;
    let version = Version::try_from(version.trim().to_string())?;
    info!("enter authors of package (separated by comma):");
    debug!("note: leave it blank if you don't want to specify it");
    let mut authors = String::new();
    std::io::stdin().read_line(&mut authors)?;
    let authors = if authors.is_empty() {
      None
    } else {
      Some(authors.split(",").map(|s| s.trim().to_string()).collect())
    };
    info!("enter description of package:");
    debug!("note: leave it blank if you don't want to specify it");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description)?;
    let description = if description.is_empty() {
      None
    } else {
      Some(description.trim().to_string())
    };

    info!("specify dependencies of package {} (separated by comma):", name.magenta().bold());
    debug!("example format of dependency: name@version/distribution, name@version/distribution, ...");
    debug!("note: leave it blank if package has no dependencies");
    let mut dependencies = String::new();
    std::io::stdin().read_line(&mut dependencies)?;
    let dependencies = if dependencies.trim().is_empty() {
      None
    } else {
      let hashmap = Some(dependencies
        .split(",")
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| {
          let mut split = s.split("@");
          let name = split.next().unwrap_or_default().trim().to_string();
          let mut split2 = split.next().unwrap_or_default().split("/");
          let version = Version::try_from(split2.next().unwrap_or_default().trim().to_string()).unwrap_or_default();
          let distribution = Distribution::try_from(split2.next().unwrap_or_default().trim().to_string()).unwrap_or_default();
          (name, ManifestDependencyData { version, distribution })
        })
        .collect::<HashMap<String, ManifestDependencyData>>()
      );
      hashmap
    };

    Ok(Self {
      package: ManifestPackage {
        name: name.trim().to_string(),
        version,
        authors,
        description
      },
      dependencies
    })
  }

  pub fn from_pwd() -> anyhow::Result<Self>
  {
    let path = std::env::current_dir()?
      .join(POPPY_MANIFEST_NAME);
    anyhow::ensure!(path.exists(), "manifest not found in pwd");

    let manifest = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&manifest)?)
  }

  pub fn save(&self) -> anyhow::Result<()>
  {
    let path = std::env::current_dir()?
      .join(POPPY_MANIFEST_NAME);
    anyhow::ensure!(!path.exists(), "manifest already exists in pwd");
    std::fs::write(&path, toml::to_string(&self)?)?;
    debug!("manifest saved to {}", path.to_str().unwrap().dimmed());
    Ok(())
  }
}