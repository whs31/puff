use std::fmt::Display;
use std::ops::Deref;
use anyhow::Context;
use log::warn;
use crate::utils::helper_types::{PlatformArch, Version};

pub struct Environment
{
  pub cmake_version: Version,
  pub arch: PlatformArch
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

