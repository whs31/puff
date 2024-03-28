use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, Default, Deserialize, Serialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Arch
{
  #[default] X86_64,
  Aarch64,
  Arm,
  ArmV5TE,
  ArmV7,
  ArmV7A,
  ArmV7R,
  ArmV8,
  Loongarch64,
  Unknown
}

impl std::fmt::Display for Arch {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::X86_64 => write!(f, "x86_64"),
      Self::Aarch64 => write!(f, "aarch64"),
      Self::Arm => write!(f, "arm"),
      Self::ArmV5TE => write!(f, "armv5te"),
      Self::ArmV7 => write!(f, "armv7"),
      Self::ArmV7A => write!(f, "armv7a"),
      Self::ArmV7R => write!(f, "armv7r"),
      Self::ArmV8 => write!(f, "armv8"),
      Self::Loongarch64 => write!(f, "loongarch64"),
      Self::Unknown => write!(f, "unknown"),
    }
  }
}

impl FromStr for Arch {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.to_lowercase();
    match s.as_str() {
      "x86_64" | "amd64" | "x64" | "86_64" | "x86-64" => Ok(Self::X86_64),
      "aarch64" | "arm64" | "arm64-v8a" | "aarch64-v8a" => Ok(Self::Aarch64),
      "arm" | "armv6" | "armv6l" | "armv6hf" => Ok(Self::Arm),
      "armv5te" | "armv5tejl" => Ok(Self::ArmV5TE),
      "armv7" | "armv7l" => Ok(Self::ArmV7),
      "armv7a" | "armv7al" => Ok(Self::ArmV7A),
      "armv7r" | "armv7rl" => Ok(Self::ArmV7R),
      "armv8" | "armv8l" => Ok(Self::ArmV8),
      "loongarch64" | "loongarch" | "loongarch64l" => Ok(Self::Loongarch64),
      "unknown"|"any" => Ok(Self::Unknown),
      _ => Err(anyhow::anyhow!("unknown arch: {}", s))
    }
  }
}

impl Arch
{
  pub fn from_env() -> anyhow::Result<Self>
  {
    let arch = whoami::arch();
    match arch {
      whoami::Arch::ArmV5 | whoami::Arch::ArmV6 => Ok(Self::Arm),
      whoami::Arch::ArmV7 => Ok(Self::ArmV7),
      whoami::Arch::Arm64 => Ok(Self::Aarch64),
      whoami::Arch::X64 => Ok(Self::X86_64),
      _ => Err(anyhow::anyhow!("unknown arch: {}", arch))
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
    let a = Arch::X86_64;
    assert_eq!(a.to_string(), "x86_64");

    let a = Arch::Aarch64;
    assert_eq!(a.to_string(), "aarch64");

    let a = Arch::Arm;
    assert_eq!(a.to_string(), "arm");

    let a = Arch::ArmV5TE;
    assert_eq!(a.to_string(), "armv5te");

    let a = Arch::ArmV7;
    assert_eq!(a.to_string(), "armv7");

    let a = Arch::ArmV7A;
    assert_eq!(a.to_string(), "armv7a");

    let a = Arch::ArmV7R;
    assert_eq!(a.to_string(), "armv7r");

    let a = Arch::ArmV8;
    assert_eq!(a.to_string(), "armv8");

    let a = Arch::Loongarch64;
    assert_eq!(a.to_string(), "loongarch64");
  }

  #[test]
  fn test_deserialize()
  {
    let a = Arch::from_str("x86_64").unwrap();
    assert_eq!(a, Arch::X86_64);

    let a = Arch::from_str("aarch64").unwrap();
    assert_eq!(a, Arch::Aarch64);

    let a = Arch::from_str("arm").unwrap();
    assert_eq!(a, Arch::Arm);

    let a = Arch::from_str("armv5te").unwrap();
    assert_eq!(a, Arch::ArmV5TE);

    let a = Arch::from_str("armv7").unwrap();
    assert_eq!(a, Arch::ArmV7);

    let a = Arch::from_str("armv7a").unwrap();
    assert_eq!(a, Arch::ArmV7A);

    let a = Arch::from_str("armv7r").unwrap();
    assert_eq!(a, Arch::ArmV7R);

    let a = Arch::from_str("armv8").unwrap();
    assert_eq!(a, Arch::ArmV8);

    let a = Arch::from_str("loongarch64").unwrap();
    assert_eq!(a, Arch::Loongarch64);

    let a = Arch::from_str("unknown");
    assert!(a.is_err());
  }
}