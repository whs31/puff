use anyhow::Context;

#[derive(Debug)]
pub struct Directories
{
  pub dirs: directories::ProjectDirs
}

impl Default for Directories { fn default() -> Self { Self::new().unwrap() } }

impl Directories
{
  pub fn new() -> anyhow::Result<Self> {
    Ok(Self {
      dirs: directories::ProjectDirs::from("io", crate::names::NAME, crate::names::NAME)
        .context("failed to get directories for config and cache!")?
    })
  }
}