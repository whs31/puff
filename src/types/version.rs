use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, Deserialize, Serialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct VersionRange
{
  pub min: Version,
  pub max: Version
}

#[derive(Debug, Copy, Clone, Hash, Deserialize, Serialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u16, u16, u16);

impl Display for VersionRange {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.min == self.max { write!(f, "={}", self.min) }
    else if self.max.is_highest() { write!(f, "^{}", self.min) }
    else { write!(f, "{}..{}", self.min, self.max) }
  }
}

impl Display for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.is_highest() { write!(f, "latest") }
    else { write!(f, "{}.{}.{}", self.0, self.1, self.2) }
  }
}

impl Default for VersionRange { fn default() -> Self { Self::latest() } }
impl Default for Version { fn default() -> Self { Self::lowest() } }

impl FromStr for VersionRange {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "any" | "latest" => Ok(Self::latest()),
      _ => {
        return if s.starts_with('=') { Ok(Self { min: s[1..].parse()?, max: s[1..].parse()? }) }
        else if s.starts_with('<') { Ok(Self { min: Version::lowest(), max: s[1..].parse()? }) }
        else if s.starts_with('^') { Ok(Self { min: s[1..].parse()?, max: Version::highest() }) }
        else { Ok(Self { min: s.parse()?, max: Version::highest() }) }
      }
    }
  }
}

impl FromStr for Version {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut v = s.split('.');
    let v = (
      v.next().ok_or_else(|| anyhow::anyhow!("invalid version"))?,
      v.next().ok_or_else(|| anyhow::anyhow!("invalid version"))?,
      v.next().ok_or_else(|| anyhow::anyhow!("invalid version"))?
    );
    Ok(Self(v.0.parse()?, v.1.parse()?, v.2.parse()?))
  }
}

impl VersionRange
{
  pub fn latest() -> Self { Self { min: Version::lowest(), max: Version::highest() } }
}

impl Version
{
  pub fn lowest() -> Self { Self(0, 0, 0) }
  pub fn highest() -> Self  { Self(u16::MAX, u16::MAX, u16::MAX) }

  pub fn is_lowest(&self) -> bool { self.0 == 0 && self.1 == 0 && self.2 == 0 }
  pub fn is_highest(&self) -> bool { self.0 == u16::MAX && self.1 == u16::MAX && self.2 == u16::MAX }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialize_version() {
    let v = Version(1, 2, 3);
    assert_eq!(v.to_string(), "1.2.3");

    let v = Version(0, 0, 0);
    assert_eq!(v.to_string(), "0.0.0");

    let v = Version(65535, 65535, 65535);
    assert_eq!(v.to_string(), "latest");
  }

  #[test]
  fn deserialize_version() {
    let v = Version::from_str("1.2.3").unwrap();
    assert_eq!(v, Version(1, 2, 3));

    let v = Version::from_str("0.0.0").unwrap();
    assert_eq!(v, Version(0, 0, 0));
    assert!(v.is_lowest());

    let v = Version::from_str("65535.65535.65535").unwrap();
    assert_eq!(v, Version(65535, 65535, 65535));
    assert!(v.is_highest());
  }

  #[test]
  fn serialize_version_range() {
    let v = VersionRange { min: Version(1, 2, 3), max: Version(4, 5, 6) };
    assert_eq!(v.to_string(), "1.2.3..4.5.6");

    let v = VersionRange { min: Version(0, 0, 0), max: Version(65535, 65535, 65535) };
    assert_eq!(v.to_string(), "^0.0.0");

    let v = VersionRange { min: Version(1, 2, 3), max: Version(1, 2, 3) };
    assert_eq!(v.to_string(), "=1.2.3");
  }

  #[test]
  fn deserialize_version_range() {
    let v = VersionRange::from_str("1.2.3").unwrap();
    assert_eq!(v, VersionRange { min: Version(1, 2, 3), max: Version::highest() });
    assert_eq!(v.min, Version(1, 2, 3));

    let v = VersionRange::from_str("0.0.0").unwrap();
    assert_eq!(v, VersionRange { min: Version(0, 0, 0), max: Version::highest() });
    assert_eq!(v.min, Version(0, 0, 0));
    assert!(v.min.is_lowest());

    let v = VersionRange::from_str("any").unwrap();
    assert_eq!(v, VersionRange { min: Version(0, 0, 0), max: Version::highest() });
    assert_eq!(v.min, Version(0, 0, 0));
    assert!(v.min.is_lowest());

    let v = VersionRange::from_str("latest").unwrap();
    assert_eq!(v, VersionRange { min: Version(0, 0, 0), max: Version::highest() });
    assert_eq!(v.min, Version(0, 0, 0));
    assert!(v.min.is_lowest());

    let v = VersionRange::from_str("=1.2.3").unwrap();
    assert_eq!(v, VersionRange { min: Version(1, 2, 3), max: Version(1, 2, 3) });
    assert_eq!(v.min, Version(1, 2, 3));
    assert_eq!(v.max, Version(1, 2, 3));

    let v = VersionRange::from_str("^0.0.0").unwrap();
    assert_eq!(v, VersionRange { min: Version(0, 0, 0), max: Version(65535, 65535, 65535) });
    assert_eq!(v.min, Version(0, 0, 0));
    assert!(v.min.is_lowest());
    assert_eq!(v.max, Version(65535, 65535, 65535));
    assert!(v.max.is_highest());

    let v = VersionRange::from_str("<1.2.3").unwrap();
    assert_eq!(v, VersionRange { min: Version(0, 0, 0), max: Version(1, 2, 3) });
    assert_eq!(v.min, Version(0, 0, 0));
    assert!(v.min.is_lowest());
    assert_eq!(v.max, Version(1, 2, 3));
    assert!(!v.max.is_highest());

    let v = VersionRange::from_str("=55.13.5532").unwrap();
    assert_eq!(v, VersionRange { min: Version(55, 13, 5532), max: Version(55, 13, 5532) });
    assert_eq!(v.min, Version(55, 13, 5532));
    assert_eq!(v.max, Version(55, 13, 5532));
  }
}