use anyhow::Context;
use log::trace;
use crate::consts::POPPY_INSTALLATION_DIRECTORY_NAME;
use crate::utils::helper_types::{PlatformArch, Version};

pub struct Environment
{
  pub cmake_version: Version,
  pub arch: PlatformArch,
  pub install_dir: String
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

    let install_dir = std::env::current_dir()?
      .join(POPPY_INSTALLATION_DIRECTORY_NAME)
      .to_str()
      .context("failed to convert path to string")?
      .to_string();

    trace!("install dir: {}", install_dir);

    Ok(Self {
      cmake_version,
      arch: PlatformArch::from_env()?,
      install_dir
    })
  }

  pub fn platform_dependent_install_path(&self) -> String
  {
    match self.arch
    {
      PlatformArch::Windows64 => self.install_dir.replace("/", "\\"),
      _ => self.install_dir.replace("\\", "/"),
    }
  }
}


