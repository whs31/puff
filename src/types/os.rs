use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, Default, Deserialize, Serialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum OperatingSystem
{
  Linux,
  Windows,
  MacOS,
  Android,
  #[default] Unknown
}

impl std::fmt::Display for OperatingSystem {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Linux => write!(f, "linux"),
      Self::Windows => write!(f, "windows"),
      Self::MacOS => write!(f, "macos"),
      Self::Android => write!(f, "android"),
      Self::Unknown => write!(f, "unknown")
    }
  }
}

impl FromStr for OperatingSystem {
  type Err = void::Void;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.to_lowercase();
    match s.as_str() {
      "linux" | "unix" | "linux-gnu" | "gnu/linux" | "gnu" => Ok(Self::Linux),
      "windows" | "win32" | "win" | "microsoft" => Ok(Self::Windows),
      "macos" | "darwin" | "mac" | "apple" => Ok(Self::MacOS),
      "android" | "android-os" | "androidos" => Ok(Self::Android),
      _ => Ok(Self::Unknown)
    }
  }
}

impl OperatingSystem
{
  pub fn from_env() -> Self
  {
    let os = whoami::platform();
    match os {
      whoami::Platform::Linux => Self::Linux,
      whoami::Platform::Windows => Self::Windows,
      whoami::Platform::MacOS => Self::MacOS,
      whoami::Platform::Android => Self::Android,
      _ => Self::Unknown
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
    let os = OperatingSystem::Linux;
    assert_eq!(os.to_string(), "linux");

    let os = OperatingSystem::Windows;
    assert_eq!(os.to_string(), "windows");

    let os = OperatingSystem::MacOS;
    assert_eq!(os.to_string(), "macos");

    let os = OperatingSystem::Android;
    assert_eq!(os.to_string(), "android");

    let os = OperatingSystem::Unknown;
    assert_eq!(os.to_string(), "unknown");
  }

  #[test]
  fn test_deserialize()
  {
    let os = OperatingSystem::from_str("linux").unwrap();
    assert_eq!(os, OperatingSystem::Linux);

    let os = OperatingSystem::from_str("windows").unwrap();
    assert_eq!(os, OperatingSystem::Windows);

    let os = OperatingSystem::from_str("macos").unwrap();
    assert_eq!(os, OperatingSystem::MacOS);

    let os = OperatingSystem::from_str("android").unwrap();
    assert_eq!(os, OperatingSystem::Android);

    let os = OperatingSystem::from_str("unknown").unwrap();
    assert_eq!(os, OperatingSystem::Unknown);

    let os = OperatingSystem::from_str("unix").unwrap();
    assert_eq!(os, OperatingSystem::Linux);

    let os = OperatingSystem::from_str("win32").unwrap();
    assert_eq!(os, OperatingSystem::Windows);

    let os = OperatingSystem::from_str("darwin").unwrap();
    assert_eq!(os, OperatingSystem::MacOS);

    let os = OperatingSystem::from_str("apple").unwrap();
    assert_eq!(os, OperatingSystem::MacOS);

    let os = OperatingSystem::from_str("android-os").unwrap();
    assert_eq!(os, OperatingSystem::Android);

    let os = OperatingSystem::from_str("androidos").unwrap();
    assert_eq!(os, OperatingSystem::Android);

    let os = OperatingSystem::from_str("microsoft").unwrap();
    assert_eq!(os, OperatingSystem::Windows);

    let os = OperatingSystem::from_str("win").unwrap();
    assert_eq!(os, OperatingSystem::Windows);

    let os = OperatingSystem::from_str("mac").unwrap();
    assert_eq!(os, OperatingSystem::MacOS);

    let os = OperatingSystem::from_str("apple").unwrap();
    assert_eq!(os, OperatingSystem::MacOS);

    let os = OperatingSystem::from_str("unix").unwrap();
    assert_eq!(os, OperatingSystem::Linux);

    let os = OperatingSystem::from_str("gnu/linux").unwrap();
    assert_eq!(os, OperatingSystem::Linux);

    let os = OperatingSystem::from_str("gnu").unwrap();
    assert_eq!(os, OperatingSystem::Linux);
  }
}