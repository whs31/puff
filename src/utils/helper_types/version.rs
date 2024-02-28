use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version
{
  pub major: u32,
  pub minor: u32,
  pub patch: u32
}

impl Version
{
  #[allow(dead_code)]
  pub fn new(major: u32, minor: u32, patch: u32) -> Self
  {
    Self
    {
      major,
      minor,
      patch
    }
  }
}

impl Default for Version
{
  fn default() -> Self
  {
    Self
    {
      major: 0,
      minor: 0,
      patch: 0
    }
  }
}

impl std::fmt::Display for Version
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
  }
}

impl TryFrom<&str> for Version
{
  type Error = anyhow::Error;

  fn try_from(s: &str) -> Result<Self, Self::Error>
  {
    let mut parts = s.split('.');
    Ok(Self
    {
      major: parts.next().unwrap_or("0").parse()?,
      minor: parts.next().unwrap_or("0").parse()?,
      patch: parts.next().unwrap_or("0").parse()?,
    })
  }
}

impl TryFrom<String> for Version
{
  type Error = anyhow::Error;

  fn try_from(s: String) -> Result<Self, Self::Error>
  {
    Self::try_from(s.as_str())
  }
}