use anyhow::Context;

pub struct Directories
{
  pub dirs: directories::ProjectDirs
}

impl Directories
{
  pub fn new() -> anyhow::Result<Self> {
    Ok(Self {
      dirs: directories::ProjectDirs::from("io", crate::names::NAME, crate::names::NAME)
        .context("failed to get directories for config and cache!")?
    })
  }
}