use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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