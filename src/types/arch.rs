use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, Default, Deserialize, Serialize)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Arch
{
  #[default]
  X86_64,
  Aarch64,
  Arm,
  ArmV5TE,
  ArmV7,
  ArmV7A,
  ArmV7R,
  ArmV8,
  Loongarch64
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
      Self::Loongarch64 => write!(f, "loongarch64")
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
      _ => Err(anyhow::anyhow!("unknown arch: {}", s))
    }
  }
}