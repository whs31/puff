use std::collections::HashMap;
use nestify::nest;
use serde::{Deserialize, Serialize};
use crate::manifest::dependency::ManifestDependencyData;
use crate::types::{VersionRange};

nest!
{
  #[derive(Debug, Clone, Serialize, Deserialize)]*
  pub struct Manifest
  {
    pub this: pub struct ManifestSectionThis {
      pub name: String,
      pub version: VersionRange,
      pub description: Option<String>,
      pub authors: Option<Vec<String>>,
      pub license: Option<String>
    },
    pub needs: Option<HashMap<String, ManifestDependencyData>>,
    pub build: Option<HashMap<String, VersionRange>>
  }
}

impl Manifest
{
  pub fn from_directory(path: &str) -> anyhow::Result<Self> {
    let path = std::path::Path::new(path);
    if !path.join(crate::names::MANIFEST_FILE).exists() {
      return Err(anyhow::anyhow!("manifest at {} is not found!", path.join(crate::names::MANIFEST_FILE).to_str().unwrap()));
    }
    let manifest = std::fs::read_to_string(path.join(crate::names::MANIFEST_FILE))?;
    Ok(toml::from_str(&manifest)?)
  }

  pub fn from_current_directory() -> anyhow::Result<Self> {
    Self::from_directory(std::env::current_dir()?
      .as_os_str()
      .to_str()
      .unwrap()
    )
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_deserialize()  {
    let string = r#"
    [this]
    name = "test"
    version = "10.0.0"
    description = "test"
    authors = ["author1", "author2"]
    license = "MIT"

    [needs]
    fmt = "10.0.0"
    qwe = "^10.0.0"
    asd = "1.20.1@shared"
    zxc = "=99.9.9@static"
    ltst = "latest@shared"
    "#;

    let m: Manifest = toml::from_str(string).unwrap();
    assert_eq!(m.this.name, "test");
    assert_eq!(m.this.version, "10.0.0".parse().unwrap());
    assert_eq!(m.this.description, Some("test".to_string()));
    assert_eq!(m.this.authors, Some(vec![String::from("author1"), String::from("author2")]));
    assert_eq!(m.this.license, Some("MIT".to_string()));
    assert_eq!(m.needs, Some(HashMap::from([
      ("fmt".to_string(), "10.0.0".parse().unwrap()),
      ("qwe".to_string(), "^10.0.0".parse().unwrap()),
      ("asd".to_string(), "1.20.1@shared".parse().unwrap()),
      ("zxc".to_string(), "=99.9.9@static".parse().unwrap()),
      ("ltst".to_string(), "latest@shared".parse().unwrap()),
    ])));
  }
}
