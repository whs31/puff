use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum PlatformArch
{
  Windows64,
  Linux64,
  Android,
  Any,
  Unknown
}

impl Default for PlatformArch
{
  fn default() -> Self
  {
    Self::Unknown
  }
}

impl PlatformArch
{
  pub fn from_env() -> anyhow::Result<Self>
  {
    let os = whoami::platform();
    let arch = whoami::arch().width()?;
    Ok(match (os, arch)
    {
      (whoami::Platform::Windows, whoami::Width::Bits64) => Self::Windows64,
      (whoami::Platform::Linux, whoami::Width::Bits64) => Self::Linux64,
      (whoami::Platform::Android, _) => Self::Android,
      _ => Self::Unknown
    })
  }

  pub fn to_short_string(&self) -> String
  {
    match self
    {
      PlatformArch::Windows64 => String::from("win64"),
      PlatformArch::Linux64 => String::from("lnx64"),
      PlatformArch::Android => String::from("android"),
      PlatformArch::Any => String::from("*"),
      PlatformArch::Unknown => String::from("?")
    }
  }
}

impl Display for PlatformArch
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", match self
    {
      Self::Windows64 => "windows-x64",
      Self::Linux64 => "linux-x64",
      Self::Android => "android",
      Self::Any => "any",
      _ => "unknown"
    })
  }
}

impl From<&str> for PlatformArch
{
  fn from(value: &str) -> Self
  {
    match value
    {
      "windows-x64" => Self::Windows64,
      "linux-x64" => Self::Linux64,
      "android" => Self::Android,
      "any" => Self::Any,
      _ => Self::Unknown
    }
  }
}