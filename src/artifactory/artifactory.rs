use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use anyhow::{Context, ensure};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use futures_util::stream::StreamExt;
use log::{debug, info, trace, warn};
use crate::args::{Args, Commands, PushArgs};
use crate::manifest::Manifest;
use crate::resolver::Dependency;
use crate::utils::Config;
use crate::utils::environment::Environment;
use crate::utils::helper_types::{Distribution, PlatformArch};

#[derive(Clone)]
pub struct Artifactory
{
  pub config: Rc<RefCell<Config>>,
  pub args: Rc<Args>,
  pub env: Rc<Environment>
}

impl Artifactory
{
  pub fn new(config: Rc<RefCell<Config>>, args: Rc<Args>, env: Rc<Environment>) -> Self
  {
    Self { config, args, env }
  }

  pub fn push(&self, manifest: &Manifest, data_path: &str) -> anyhow::Result<()>
  {
    debug!("pushing!");
    ensure!(self.args.arch.as_ref().is_some() && !self.args.arch.as_ref().unwrap().is_empty(), "push arch cannot be empty");
    ensure!(!self.config.borrow().auth.username.is_empty() && !self.config.borrow().auth.token.is_empty(),
      "no username or token provided for artifactory oauth. please provide using --username and \
      --token flags or enter credentials interactively via poppy --sync");

    let arch = PlatformArch::from(self.args.arch
      .clone()
      .unwrap()
      .as_str()
    );
    let distribution = Distribution::from(
      match &self.args.command
      {
        Some(Commands::Push(arg)) => arg.distribution.as_ref().context("distribution cannot be empty")?.as_str(),
        _ => "unknown"
      }
    );
    ensure!(!matches!(distribution, Distribution::Unknown), "unknown distribution");
    let push_target = match self.args.command.as_ref().context("command not found")? {
      Commands::Push(arg) => arg.name.as_ref().context("push target cannot be empty")?,
      _ => std::process::exit(1)
    };

    trace!("artifactory base url: {}", &self.config.borrow().remotes.artifactory_url);

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

    let url = strfmt::strfmt(&self.config.borrow().remotes.artifactory_url, &fmt)
      .context("failed to format artifactory url")?;

    let data = std::fs::read(data_path)?;

    let force_push = matches!(self.args.command, Some(Commands::Push(PushArgs { force: true, .. }, ..)));
    if force_push { warn!("force pushing. this can be dangerous!"); }
    trace!("pushing {} kb to {}", data.len() / 1024, url);
    trace!("username: {}", self.config.borrow().auth.username.as_str());
    trace!("token: {}", self.config.borrow().auth.token.as_str());

    let client = reqwest::blocking::Client::builder()
      .build()?;

    let exists = client
      .get(url.clone())
      .basic_auth(self.config.borrow().auth.username.as_str(), Some(self.config.borrow().auth.token.as_str()))
      .send()?;

    if exists.status() != 404 {
      info!("file already exists on artifactory");
      if !force_push {
        debug!("skipping push");
        return Ok(());
      }
      else {
        warn!("OVERRIDING PREVIOUS PACKAGE ON ARTIFACTORY!");
      }
    }
    let res = client
      .put(url)
      .basic_auth(self.config.borrow().auth.username.as_str(), Some(self.config.borrow().auth.token.as_str()))
      .body(data.to_vec())
      .send()?;
    trace!("status: {}", res.status());

    info!("pushing done!");
    Ok(())
  }

  #[tokio::main]
  pub async fn pull(&self, dep: &Dependency) -> anyhow::Result<Vec<u8>>
  {
    trace!("pulling dependency {} from artifactory", dep.name.blue().bold());
    let client = reqwest::Client::builder()
      //.redirect(reqwest::redirect::Policy::none())
      .build()?;
    let mut fmt: HashMap<String, String> = HashMap::new();
    fmt.insert("name".to_string(), dep.name.clone());
    fmt.insert("major".to_string(), dep.version.clone().major.to_string());
    fmt.insert("minor".to_string(), dep.version.clone().minor.to_string());
    fmt.insert("patch".to_string(), dep.version.clone().patch.to_string());
    fmt.insert("arch".to_string(), dep.arch.clone().to_string());
    fmt.insert("distribution".to_string(), dep.distribution.clone().to_string());
    let url = strfmt::strfmt(self.config.borrow().remotes.artifactory_url.as_str(), &fmt)
      .context("failed to format artifactory url")?;
    let api_url = strfmt::strfmt(self.config.borrow().remotes.artifactory_api_url.as_str(), &fmt)
      .context("failed to format artifactory api url")?;

    trace!("url: {}", url);
    trace!("api url: {}", api_url);

    let result = client
      .get(url.as_str())
      .basic_auth(self.config.borrow().auth.username.as_str(), Some(self.config.borrow().auth.token.as_str()))
      .send()
      .await?;
    debug!("response status: {}", result.status());
    ensure!(result.status().is_success(), "pulling from artifactory failed with status code {}", result.status().as_str());
    let total = result
      .content_length()
      .unwrap_or(1);
    debug!("total size: {} KB", total / 1024);
    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::with_template(
      "{wide_msg} {spinner:.green} {bar:30.yellow/white} {human_pos:4}/ {human_len:4} ({percent:3}%)"
    )
      .expect("setting progress bar style should not fail!")
      .progress_chars("▃ ")
    );
    pb.set_draw_target(ProgressDrawTarget::stdout_with_hz(5));
    pb.set_message("pulling artifact...");

    let mut downloaded: u64 = 0;
    let mut stream = result.bytes_stream();
    let mut data: Vec<u8> = Vec::new();
    while let Some(item) = stream.next().await {
      let chunk = item?;
      data.extend_from_slice(&chunk);
      downloaded = std::cmp::min(downloaded + chunk.len() as u64, total);
      pb.set_position(downloaded);
    }
    pb.finish_with_message("downloaded successfully");

    trace!("checking checksum...");
    let md5 = md5::compute(&data);
    trace!("md5: {:x}", md5);

    let checksum = client
      .get(api_url)
      .basic_auth(self.config.borrow().auth.username.as_str(), Some(self.config.borrow().auth.token.as_str()))
      .send()
      .await?
      .text()
      .await?;
    let json: serde_json::Value = serde_json::from_str(checksum.as_str())?;
    let md5_from_api = json
      .get("checksums")
      .and_then(|checksums| checksums.get("md5"))
      .and_then(|checksum| checksum.as_str())
      .context("checksum not found in api response")?;
    trace!("api checksum: {}", md5_from_api);
    if md5_from_api != format!("{:x}", md5) {
      warn!("checksum mismatch");
    } else {
      debug!("md5 checksum match: {}", "OK".to_string().green());
    }
    Ok(data)
  }

  #[tokio::main]
  pub async fn query(&self, query: &str) -> anyhow::Result<String>
  {
    let client = reqwest::Client::builder()
      .build()?;
    trace!("querying artifactory to {}", self.config.borrow().remotes.artifactory_aql_url.as_str().dimmed());
    let result = client
      .post(self.config.borrow().remotes.artifactory_aql_url.as_str())
      .basic_auth(self.config.borrow().auth.username.as_str(), Some(self.config.borrow().auth.token.as_str()))
      .body(String::from(query))
      .send()
      .await?;
    trace!("query response status code: {}", result.status());
    Ok(result.text().await?)
  }
}

fn save_to(path: &str, data: &[u8]) -> anyhow::Result<()> {
  trace!("saving {} bytes to {}", data.len(), path);
  std::fs::create_dir_all(Path::new(path).parent().unwrap())?;
  std::fs::write(path, data)?;
  Ok(())
}

pub trait SaveAs
{
  fn save_as(&self, path: &str) -> anyhow::Result<()>;
}

impl SaveAs for Vec<u8>
{
  fn save_as(&self, path: &str) -> anyhow::Result<()> { save_to(path, self) }
}

pub fn unpack_to(from: &str, to: &str) -> anyhow::Result<()> {
  trace!("unpacking {} to {}...",
    Path::new(from)
      .file_name()
      .context("failed to get filename for tar.gz")?
      .to_str()
      .context("failed to convert filename to str")?,
    Path::new(to)
      .file_name()
      .context("failed to get filename for tar.gz")?
      .to_str()
      .context("failed to convert filename to str")?
  );
  std::fs::create_dir_all(to)?;

  let tar_gz = std::fs::File::open(from)?;
  let tar = flate2::read::GzDecoder::new(tar_gz);
  let mut archive = tar::Archive::new(tar);
  archive.unpack(to)?;

  trace!("unpacking {} to {}... OK!",
    Path::new(from)
      .file_name()
      .context("failed to get filename for tar.gz")?
      .to_str()
      .context("failed to convert filename to str")?,
    Path::new(to)
      .file_name()
      .context("failed to get filename for tar.gz")?
      .to_str()
      .context("failed to convert filename to str")?
  );
  Ok(())
}