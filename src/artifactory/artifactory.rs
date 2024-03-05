use std::collections::HashMap;
use anyhow::{Context, ensure};
use colored::Colorize;
use log::{debug, info, trace, warn};
use crate::manifest::Manifest;
use crate::utils::Config;
use crate::utils::environment::Environment;
use crate::utils::helper_types::{Distribution, PlatformArch};

pub struct Artifactory
{
  pub config: Config,
  pub args: crate::Args,
  pub env: Environment
}

impl Artifactory
{
  pub fn new(config: &Config, args: &crate::Args, env: &Environment) -> Self
  {
    Self
    {
      config: config.clone(),
      args: args.clone(),
      env: env.clone()
    }
  }

  pub fn push(&self, manifest: &Manifest, data_path: &str) -> anyhow::Result<()>
  {
    debug!("pushing!");
    ensure!(self.args.push.as_ref().is_some() && !self.args.push.as_ref().unwrap().is_empty(), "push target cannot be empty");
    ensure!(self.args.arch.as_ref().is_some() && !self.args.arch.as_ref().unwrap().is_empty(), "push arch cannot be empty");
    ensure!(self.args.distribution.as_ref().is_some() && !self.args.distribution.as_ref().unwrap().is_empty(), "push distribution cannot be empty");
    ensure!(!self.config.auth.username.is_empty() && !self.config.auth.token.is_empty(),
      "no username or token provided for artifactory oauth. please provide using --username and \
      --token flags or enter credentials interactively via poppy --sync");

    let arch = PlatformArch::from(self.args.arch
      .clone()
      .unwrap()
      .as_str()
    );
    let distribution = Distribution::from(self.args.distribution
      .clone()
      .unwrap()
      .as_str()
    );
    let push_target = self.args.push
      .clone()
      .context("empty push target!")?;

    trace!("artifactory base url: {}", &self.config.remotes.artifactory_url);

    debug!("package name: {}", manifest.package.name.yellow().bold());
    debug!("package version: {}", manifest.package.version.to_string().cyan().bold());
    debug!("package tarball: {}", &push_target.purple().bold());
    debug!("arch: {}", &arch.to_string().green().bold());
    debug!("distribution: {}", &distribution.to_string().magenta().bold());

    let mut fmt: HashMap<String, String> = HashMap::new();
    fmt.insert("name".to_string(), manifest.package.name.clone());
    fmt.insert("major".to_string(), manifest.package.version.clone().major.to_string());
    fmt.insert("minor".to_string(), manifest.package.version.clone().minor.to_string());
    fmt.insert("patch".to_string(), manifest.package.version.clone().patch.to_string());
    fmt.insert("arch".to_string(), arch.clone().to_string());
    fmt.insert("distribution".to_string(), distribution.clone().to_string());

    let url = strfmt::strfmt(&self.config.remotes.artifactory_url, &fmt)
      .context("failed to format artifactory url")?;

    let data = std::fs::read(data_path)?;

    if self.args.force.clone() { warn!("force pushing. this can be dangerous!"); }
    trace!("pushing {} kb to {}", data.len() / 1024, url);
    trace!("username: {}", self.config.auth.username.as_str());
    trace!("token: {}", self.config.auth.token.as_str());

    let client = reqwest::blocking::Client::builder()
      .build()?;

    let exists = client
      .get(url.clone())
      .basic_auth(self.config.auth.username.as_str(), Some(self.config.auth.token.as_str()))
      .send()?;

    if exists.status() != 404 {
      info!("file already exists on artifactory");
      if !self.args.force {
        debug!("skipping push");
        return Ok(());
      }
      else {
        warn!("OVERRIDING PREVIOUS PACKAGE ON ARTIFACTORY!");
      }
    }
    let res = client
      .put(url)
      .basic_auth(self.config.auth.username.as_str(), Some(self.config.auth.token.as_str()))
      .body(data.to_vec())
      .send()?;
    trace!("status: {}", res.status());

    info!("pushing done!");
    Ok(())
  }
}