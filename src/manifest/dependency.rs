use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::types::{Distribution, VersionRange};

#[derive(Debug, Clone, Copy, Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ManifestDependencyData
{
  pub version: VersionRange,
  pub distribution: Distribution
}

impl Display for ManifestDependencyData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}@{}", self.version, self.distribution)
  }
}

impl FromStr for ManifestDependencyData {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut s = s.split('@');
    if s.clone().collect::<Vec<_>>().len() == 1 {
      Ok(Self { version: s.next().unwrap().parse()?, distribution: Distribution::Shared })
    } else {
      Ok(Self { version: s.next().unwrap().parse()?, distribution: s.next().unwrap().parse()? })
    }
  }
}

impl Serialize for ManifestDependencyData {
  fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    self.to_string().serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for ManifestDependencyData {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_deserialize()  {
    let d = ManifestDependencyData::from_str("10.0.0").unwrap();
    assert_eq!(d, ManifestDependencyData { version: "10.0.0".parse().unwrap(), distribution: Distribution::Shared });

    let d = ManifestDependencyData::from_str("10.0.0@static").unwrap();
    assert_eq!(d, ManifestDependencyData { version: "10.0.0".parse().unwrap(), distribution: Distribution::Static });

    let d = ManifestDependencyData::from_str("10.0.0@shared").unwrap();
    assert_eq!(d, ManifestDependencyData { version: "10.0.0".parse().unwrap(), distribution: Distribution::Shared });

    let d = ManifestDependencyData::from_str("^10.0.0").unwrap();
    assert_eq!(d, ManifestDependencyData { version: "10.0.0".parse().unwrap(), distribution: Distribution::Shared });

    let d = ManifestDependencyData::from_str("=10.0.0@shared").unwrap();
    assert_eq!(d, ManifestDependencyData { version: "=10.0.0".parse().unwrap(), distribution: Distribution::Shared });

    let d = ManifestDependencyData::from_str("=10.0.0@static").unwrap();
    assert_eq!(d, ManifestDependencyData { version: "=10.0.0".parse().unwrap(), distribution: Distribution::Static });
  }

  #[test]
  fn test_serde_ser() {
    let d = ManifestDependencyData { version: "10.0.0".parse().unwrap(), distribution: Distribution::Shared };
    let s = serde_json::to_string(&d).unwrap();
    assert_eq!(s, "\"10.0.0@shared\"");

    let d = ManifestDependencyData { version: "=10.0.0".parse().unwrap(), distribution: Distribution::Static };
    let s = serde_json::to_string(&d).unwrap();
    assert_eq!(s, "\"=10.0.0@static\"");
  }

  #[test]
  fn test_serde_de() {
    let d = ManifestDependencyData { version: "10.0.0".parse().unwrap(), distribution: Distribution::Shared };
    let s = serde_json::to_string(&d).unwrap();
    let d_: ManifestDependencyData = serde_json::from_str(&s).unwrap();
    assert_eq!(d, d_);

    let d = ManifestDependencyData { version: "=10.0.0".parse().unwrap(), distribution: Distribution::Static };
    let s = serde_json::to_string(&d).unwrap();
    let d_: ManifestDependencyData = serde_json::from_str(&s).unwrap();
    assert_eq!(d, d_);

    let d = ManifestDependencyData { version: "10.0.0".parse().unwrap(), distribution: Distribution::Shared };
    let s = serde_json::to_string(&d).unwrap();
    let d_: ManifestDependencyData = serde_json::from_str(&s).unwrap();
    assert_eq!(d, d_);
  }
}