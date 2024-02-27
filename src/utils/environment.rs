use std::fmt::Display;
use std::ops::Deref;
use anyhow::Context;
use log::warn;
use crate::utils::helper_types::Version;

pub struct Environment
{
  pub cmake_version: Version,
  pub arch: PlatformArch
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlatformArch
{
  Windows64,
  Linux64,
  Android,
  Any,
  Unknown
}

impl Environment
{
  pub fn from_current_environment() -> anyhow::Result<Self>
  {
    let cmake_version = Version::try_from(
      String::from_utf8(
      std::process::Command::new("cmake")
        .arg("--version")
        .output()?
        .stdout
      )?
        .split('\n')
        .collect::<Vec<_>>()
        .first()
        .context("cmake version parsing failure")?
        .to_string()
        .chars()
        .filter(|c| c.is_digit(10) || *c == '.' )
        .collect::<String>()
    )?;
    Ok(Self {
      cmake_version,
      arch: PlatformArch::from_env()?
    })
  }
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