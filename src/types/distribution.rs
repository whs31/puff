use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, Default, Deserialize, Serialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Distribution
{
  Static,
  #[default] Shared,
  Unknown
}

impl std::fmt::Display for Distribution {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Static => write!(f, "static"),
      Self::Shared => write!(f, "shared"),
      Self::Unknown => write!(f, "unknown"),
    }
  }
}

impl FromStr for Distribution {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.to_lowercase();
    match s.as_str() {
      "static" => Ok(Self::Static),
      "shared"|"dynamic"|"dyn" => Ok(Self::Shared),
      "unknown"|"sources"|"any" => Ok(Self::Unknown),
      _ => Err(anyhow::anyhow!("unknown distribution: {}", s))
    }
  }
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_serialize()
  {
    let d = Distribution::Static;
    assert_eq!(d.to_string(), "static");

    let d = Distribution::Shared;
    assert_eq!(d.to_string(), "shared");
  }

  #[test]
  fn test_deserialize()
  {
    let d = Distribution::from_str("static").unwrap();
    assert_eq!(d, Distribution::Static);

    let d = Distribution::from_str("shared").unwrap();
    assert_eq!(d, Distribution::Shared);

    let d = Distribution::from_str("dynamic").unwrap();
    assert_eq!(d, Distribution::Shared);

    let d = Distribution::from_str("dyn").unwrap();
    assert_eq!(d, Distribution::Shared);

    let d = Distribution::from_str("StAtIc").unwrap();
    assert_eq!(d, Distribution::Static);

    assert!(Distribution::from_str("unknown").is_err());
  }
}
