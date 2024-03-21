use std::collections::HashMap;
use std::rc::Rc;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use crate::core;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config
{
  #[serde(skip_serializing, skip_deserializing)]
  pub directories: Rc<core::Directories>,
  pub registry: RegistryConfig,
  pub toolchain: ToolchainConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegistryConfig
{
  pub list: Vec<RegistryData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryData
{
  pub name: String,
  pub base_url: String,
  pub pattern: String,
  pub auth: Option<RegistryAuth>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegistryAuth
{
  pub username: String,
  pub password: String,
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ToolchainConfig
{
  pub cmake: CMakeConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CMakeConfig
{
  pub configure_command: String,
  pub configure_additional_definitions: HashMap<String, String>,
}

impl Default for CMakeConfig
{
  fn default() -> Self
  {
    Self
    {
      configure_command: String::from("cmake"),
      configure_additional_definitions: HashMap::new()
    }
  }
}

impl Default for RegistryData
{
  fn default() -> Self
  {
    Self
    {
      name: String::new(),
      base_url: String::new(), // http://uav.radar-mms.com/artifactory/{name}
      pattern: String::from("parcels/{org}/{name}/{version}/{name}-{version}-{arch}-{platform}-{dist}.tar.gz"),
      auth: None
    }
  }
}

impl Config
{
  pub fn create_or_load() -> anyhow::Result<Self>
  {
    let directories = Rc::new(core::Directories::new()?);
    if directories.dirs.config_dir().join("config.toml").exists() {
      Self::load()
    }
    else {
      let mut config = Self::default();
      config.directories = directories;
      config.save()?;
      Ok(config)
    }
  }

  pub fn save(&self) -> anyhow::Result<()>
  {
    let config = toml::to_string(&self)?;
    std::fs::create_dir_all(self.directories.dirs.config_dir())?;
    std::fs::write(self.directories.dirs.config_dir().join("config.toml"), config)?;
    println!("saved configuration file to {}", self.directories.dirs
      .config_dir()
      .join("config.toml")
      .to_str()
      .unwrap()
      .to_string()
      .dimmed()
      .cyan()
    );
    Ok(())
  }

  pub fn load() -> anyhow::Result<Self>
  {
    let directories = Rc::new(core::Directories::new()?);
    let config = std::fs::read_to_string(directories.dirs.config_dir().join("config.toml"))?;
    let mut config: Self = toml::from_str(&config)?;
    config.directories = directories;
    Ok(config)
  }
}