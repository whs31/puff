use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::types::{Distribution, VersionRange};

#[derive(Debug, Clone, Copy, Hash, Serialize, Deserialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ManifestDependencyData // todo: deserialize it from either string or struct
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
    // deserializes exactly like version range but optionally with distribution
    // (separated by '@'). if distribution is not provided, it defaults to shared
    // e.g.: "10.0.0", "10.0.0@static", "10.0.0@shared", "^10.0.0", "=10.0.0@shared"

    let mut s = s.split('@');
    if s.clone().collect::<Vec<_>>().len() == 1 {
      Ok(Self { version: s.next().unwrap().parse()?, distribution: Distribution::Shared })
    } else {
      Ok(Self { version: s.next().unwrap().parse()?, distribution: s.next().unwrap().parse()? })
    }
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
}