use std::path::Path;
use colored::Colorize;
use log::{debug, trace};
use crate::utils::global::PROJECT_DIRS;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config
{
  pub remotes: ConfigRemote,
  pub auth: ConfigAuth
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ConfigRemote
{
  pub registry_url: String,
  pub ci_url: String,
  pub artifactory_url: String
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct ConfigAuth
{
  pub username: String,
  pub token: String
}

impl Default for Config
{
  fn default() -> Self {
    Self {
      remotes: ConfigRemote::default(),
      auth: ConfigAuth::default()
    }
  }
}

impl Default for ConfigRemote
{
  fn default() -> Self {
    Self {
      registry_url: String::from("http://uav.radar-mms.com/gitlab/test/essentials/poppy/poppy-registry.git"),
      ci_url: String::from("http://uav.radar-mms.com/gitlab/test/essentials/ci.git"),
      artifactory_url: String::from("") // todo
    }
  }
}

impl Config
{
  pub fn create_or_load() -> anyhow::Result<Self>
  {
    trace!("initializing config");
    let dirs = PROJECT_DIRS.lock().unwrap();
    let path = Path::new(dirs.config_dir()).join("config.toml");
    if path.exists() {
      debug!("found existing config in {}", path.display());
      let contents = std::fs::read_to_string(&path)?;
      let config: Self = toml::from_str(&contents)?;
      config.print();
      Ok(config)
    } else {
      debug!("creating new default config in {}", path.display());
      let config = Self::default();
      let contents = toml::to_string(&config)?;
      std::fs::create_dir_all(&path.parent().unwrap())?;
      std::fs::write(&path, contents)?;
      debug!("created config file in {}", path.display());
      config.print();
      Ok(config)
    }
  }

  fn print(&self)
  {
    debug!("registry url:    {}", self.remotes.registry_url.blue());
    debug!("ci url:          {}", self.remotes.ci_url.blue());
    debug!("artifactory url: {}", self.remotes.artifactory_url.blue());

    debug!("username: {}", if self.auth.username.is_empty() { "empty".red() } else { format!("**{}**", &self.auth.username[2..5]).green() });
    debug!("token: {}", if self.auth.token.is_empty() { "empty".red() } else { "********".green() });
  }
}
