use std::rc::Rc;
use std::time::Duration;
use anyhow::{bail, Context};
use colored::Colorize;
use indicatif::ProgressBar;

pub struct Artifactory
{
  pub name: String,
  pub url_format: String,
  pub username: Option<String>,
  pub token: Option<String>,
  url_ping: String,
  config: Rc<crate::core::Config>
}

impl Artifactory
{
  pub fn new(config: Rc<crate::core::Config>, name: &str) -> anyhow::Result<Self>
  {
    let name = if config.registry.list
      .iter()
      .find(|x| x.name == name)
      .is_some()
    {
      name.to_string()
    } else {
      bail!("internal error: no such registry in config: {}. report this bug to the developers", name)
    };
    let reg_data = config.registry.list
      .iter()
      .find(|x| x.name == name)
      .context("internal error: no such registry in config: {}. report this bug to the developers")?;
    let url_format = format!("{}{}{}/{}",
      reg_data.base_url,
      if reg_data.base_url.ends_with('/') { "" } else { "/" },
      reg_data.name,
      reg_data.pattern
    );
    let url_ping = format!("{}{}{}",
      reg_data.base_url,
      if reg_data.base_url.ends_with('/') { "" } else { "/" },
      reg_data.name
    );
    let username = match &reg_data.auth {
      None => None,
      Some(x) => Some(x.username.clone())
    };
    let token = match &reg_data.auth {
      None => None,
      Some(x) => Some(x.password.clone())
    };
    Ok(Artifactory
    {
      name,
      url_format,
      url_ping,
      username,
      token,
      config
    })
  }

  pub fn ping(&self) -> anyhow::Result<()>
  {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(format!("checking access to {}",
      &self.name.bold().bright_green()
    ));
    let client = reqwest::blocking::Client::new();
    let res = client
      .get(&self.url_ping)
      .basic_auth(self.username.as_ref().unwrap_or(&"guest".to_string()), self.token.clone())
      .send()?;
    if !res.status().is_success() {
      bail!("ping failed: {}", res.status());
    }
    pb.finish_with_message(format!("{} {}",
      &self.name.bold().magenta(),
      "is available".to_string().green().bold(),
    ));
    Ok(())
  }
}

