use std::collections::HashMap;
use std::rc::Rc;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use crate::core;
use crate::core::args::{Command, RegistryCommand, ToolchainCommand};

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

  pub fn process_args(&mut self, args: &core::Args) -> anyhow::Result<()>
  {
    match &args.command {
      None => { return Ok(()) }
      Some(command) => {
        match command {
          Command::Registry(registry_command) => {
            match registry_command {
              RegistryCommand::Add(a) => {
                let mut reg_data = RegistryData {
                  name: a.name.clone(),
                  base_url: a.url.clone(),
                  pattern: a.pattern.clone(),
                  auth: None
                };

                if let Some(u) = &a.username {
                  reg_data.auth = Some(RegistryAuth {
                    username: u.clone(),
                    password: a.token.clone().unwrap_or(String::new())
                  });
                }

                self.registry.list.push(reg_data);
                self.save()?;
                println!("added registry {} to config", a.name.yellow().bold());
                Ok(())
              }
              RegistryCommand::Remove(a) => {
                return if let Some(i) = self.registry.list.iter().position(|x| x.name == a.name) {
                  self.registry.list.remove(i);
                  self.save()?;
                  println!("removed registry {} from config", a.name.yellow().bold());
                  Ok(())
                } else {
                  Err(anyhow::anyhow!("registry {} not found in config", a.name))
                }
              }
            }
          }
          Command::Toolchain(toolchain_command) => {
            match toolchain_command {
              ToolchainCommand::Cmake(a) => {
                if let Some(args) = &a.configure_args {
                  let configure_additional_definitions = args
                    .iter()
                    .map(|x| {
                      let x = x.trim();
                      let mut parts = x.splitn(2, '=');
                      let key = parts.next().unwrap();
                      let value = parts.next().unwrap_or("");
                      (key.to_string(), value.to_string())
                    })
                    .collect::<HashMap<String, String>>();
                  self.toolchain.cmake.configure_additional_definitions = configure_additional_definitions;
                };
                if let Some(cmake) = &a.configure_command {
                  self.toolchain.cmake.configure_command = cmake.clone();
                }
                self.save()?;
                println!("saved cmake configuration to config");
                Ok(())
              },
              _ => { return Ok(()) }
            }
          }
          _ => { return Ok(()) }
        }
      }
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