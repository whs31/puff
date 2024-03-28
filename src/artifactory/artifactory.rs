use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;
use anyhow::{bail, Context, ensure};
use colored::Colorize;
use indicatif::ProgressBar;
use crate::types::{Arch, Distribution, OperatingSystem};

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

  pub fn push(
    &self,
    path: &str,
    packed_file: &str,
    distribution: Distribution,
    arch: Arch,
    os: OperatingSystem,
    force: bool
  ) -> anyhow::Result<()>
  {
    let manifest = crate::manifest::Manifest::from_directory(path)?;
    let recipe = crate::builder::Recipe::from_directory(path)?;
    println!("pushing {}@{} to {}", manifest.this.name, manifest.this.version, &self.name);

    let mut fmt: HashMap<String, String> = HashMap::new();
    fmt.insert("name".to_string(), manifest.this.name.clone());
    fmt.insert("version".to_string(), manifest.this.version.clone().to_string());
    fmt.insert("arch".to_string(), arch.to_string());
    fmt.insert("platform".to_string(), os.to_string());
    fmt.insert("distribution".to_string(), distribution.to_string());

    let url = strfmt::strfmt(self.url_format.as_str(), &fmt)
      .context("failed to format url")?;
    let client = reqwest::blocking::Client::builder()
      .build()?;
    let exists = client
      .get(url.clone())
      .basic_auth(self.username.as_ref().unwrap_or(&"guest".to_string()), self.token.clone())
      .send()?;

    if exists.status().is_success() {
      println!("{} {}@{}/{}/{}/{} {} {}",
        String::from("package").yellow().bold(),
        manifest.this.name.bold().magenta(),
        manifest.this.version.to_string().bold().green(),
        distribution.to_string().cyan(),
        arch.to_string().white(),
        os.to_string().white(),
        String::from("already exists in").yellow().bold(),
        &self.name.bold().cyan()
      );

      if !force {
        println!("{}. use --force flag to push anyway", String::from("warning").cyan().bold());
        return Ok(());
      } else {
        println!("{}", String::from("warning: overriding existing package").yellow().bold());
      }
    }

    let res = client
      .put(url)
      .basic_auth(self.username.as_ref().unwrap_or(&"guest".to_string()), self.token.clone())
      .body(std::fs::read(packed_file)?)
      .send()?;

    if !res.status().is_success() {
      bail!("failed to push package: {}", res.status());
    }
    std::fs::remove_file(packed_file)?;
    Ok(())
  }
}

