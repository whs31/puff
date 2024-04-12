use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Duration;
use anyhow::{anyhow, bail, Context, ensure};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::stream::StreamExt;
use crate::artifactory::entry::Entry;
use crate::resolver::{Dependency, PackageGet};
use crate::types::{Arch, Distribution, OperatingSystem};

pub struct Artifactory
{
  pub name: String,
  pub url_format: String,
  pub username: Option<String>,
  pub token: Option<String>,
  url_ping: String,
  url_aql: String,
  url_api_format: String,
  config: Rc<crate::core::Config>,
  pub(crate) available_packages: Vec<Entry>
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
    let url_api_format = format!("{}{}api/storage/{}/{}",
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
      url_api_format,
      url_ping,
      url_aql: format!("{}{}api/search/aql", reg_data.base_url, if reg_data.base_url.ends_with('/') { "" } else { "/" }),
      username,
      token,
      config,
      available_packages: Vec::new()
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

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(format!("pushing {}@{}/{}/{}/{} to {}",
      &manifest.this.name.bold().magenta(),
      &manifest.this.version.to_string().bold().green(),
      distribution.to_string().cyan().dimmed(),
      arch.to_string().white().dimmed(),
      os.to_string().white().dimmed(),
      self.name.bold().bright_green()
    ));

    let mut fmt: HashMap<String, String> = HashMap::new();
    fmt.insert("name".to_string(), manifest.this.name.clone());
    fmt.insert("version".to_string(), manifest.this.version.clone().to_string());
    fmt.insert("arch".to_string(), arch.to_string());
    fmt.insert("platform".to_string(), os.to_string());
    fmt.insert("dist".to_string(), distribution.to_string());

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
        pb.finish_and_clear();
        println!("{}: use --force flag to push anyway", String::from("tip").cyan().bold());
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

    pb.finish_with_message(format!("{} {}@{}/{}/{}/{} to {}",
      "successfully pushed".to_string().green().bold(),
      &manifest.this.name.bold().magenta(),
      &manifest.this.version.to_string().bold().green(),
      distribution.to_string().cyan().dimmed(),
      arch.to_string().white().dimmed(),
      os.to_string().white().dimmed(),
      &self.name.bold().cyan()
    ));
    Ok(())
  }

  #[tokio::main]
  pub async fn query(&self, query: &str) -> anyhow::Result<String>
  {
    let client = reqwest::Client::builder()
      .build()?;
    let result = client
      .post(&self.url_aql)
      .basic_auth(self.username.as_ref().unwrap_or(&"guest".to_string()), self.token.clone())
      .body(String::from(query))
      .send()
      .await?;
    if !result.status().is_success() {
      return Err(anyhow!("artifactory is not responding"));
    }
    Ok(result.text().await?)
  }

  pub fn sync_aql(&mut self) -> anyhow::Result<&mut Self>
  {
    let raw = self.query(
      format!(r#"items.find({{"repo": "{name}", "name": {{"$match": "*"}}}}).sort({{"$desc": ["created"]}})"#, name = self.name).as_str()
    )?;

    let items = serde_json::from_str::<crate::artifactory::query::PackageQueryResponse>(&raw)?;
    let mut packages: Vec<Entry> = Vec::new();

    for item in items.results {
      packages.push(Entry::new(Dependency::from_package_name(&item.name)?, &self.url_format, &self.url_api_format)?);
    }

    self.available_packages = packages;
    Ok(self)
  }
}

impl PackageGet for Artifactory
{
  #[tokio::main]
  async fn get(&self, dependency: &Dependency, allow_sources: bool) -> anyhow::Result<PathBuf>
  {
    let dep = self.latest_satisfied(dependency, allow_sources)?;
    let entry = self.available_packages
      .iter()
      .find(|x| x.dependency == dep)
      .context("fatal failure in resolving packages. contact the maintainers")?
      .clone();

    let client = reqwest::Client::builder()
      .build()?;
    let result = client
      .get(&entry.url)
      .basic_auth(self.username.as_ref().unwrap_or(&"guest".to_string()), self.token.clone())
      .send()
      .await?;
    ensure!(result.status().is_success(), "pulling from artifactory failed with status code {}", result.status().as_str());
    let total = result
      .content_length()
      .unwrap_or(1);
    let pb = ProgressBar::new(total / 1024);
    pb.set_style(
      ProgressStyle::with_template("{spinner:.green} {wide_msg} [{elapsed}] [{bar:30.blue/blue}] {human_pos:4}/{human_len:4} kb ({percent:3})")
        .unwrap()
        .progress_chars("█▒░")
    );
    pb.set_message(format!("downloading {}@{}/{}/{}/{}",
      &entry.dependency.name.bold().magenta(),
      &entry.dependency.version.to_string().green(),
      &entry.dependency.arch.to_string().dimmed(),
      &entry.dependency.os.to_string().dimmed(),
      &entry.dependency.distribution.to_string().dimmed()
    ));

    let target_path = self.config
      .directories
      .dirs
      .cache_dir()
      .join(format!("{}-{}-{}-{}-{}.tar.gz",
        entry.dependency.name,
        entry.dependency.version.to_string(),
        entry.dependency.arch.to_string(),
        entry.dependency.os.to_string(),
        entry.dependency.distribution.to_string()
      ));
    let mut downloaded: u64 = 0;
    let mut stream = result.bytes_stream();
    let mut data: Vec<u8> = Vec::new();
    while let Some(item) = stream.next().await {
      let chunk = item?;
      data.extend_from_slice(&chunk);
      downloaded = std::cmp::min(downloaded + chunk.len() as u64, total);
      pb.set_position(downloaded / 1024);
    }
    pb.finish_and_clear();
    let md5 = md5::compute(&data);

    let checksum = client
      .get(&entry.api_url)
      .basic_auth(self.username.as_ref().unwrap_or(&"guest".to_string()), self.token.clone())
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
    ensure!(md5_from_api == format!("{:x}", md5), "checksum mismatch");
    let mut file = std::fs::File::create(&target_path)?;
    file.write_all(&data)?;
    Ok(target_path)
  }

  fn latest_satisfied(&self, dependency: &Dependency, allow_sources: bool) -> anyhow::Result<Dependency>
  {
    let pb = ProgressBar::new_spinner()
      .with_message(format!("searching for {}", dependency));
    let valid_versions = self.available_packages
      .iter()
      .cloned()
      .filter(|x| x.dependency.ranged_compare(dependency))
      .collect::<Vec<_>>();
    let mut entry = valid_versions
      .iter()
      .cloned()
      .max_by(|a, b| a.dependency.version.cmp(&b.dependency.version));

    let mut is_source = false;
    if entry.is_none() && allow_sources {
      let src = dependency.as_sources_dependency();
      let source_valid_versions = self.available_packages
        .iter()
        .cloned()
        .filter(|x| x.dependency.ranged_compare(&src))
        .collect::<Vec<_>>();
      entry = source_valid_versions
        .iter()
        .cloned()
        .max_by(|a, b| a.dependency.version.cmp(&b.dependency.version));
      is_source = true;
    }
    let entry = entry;
    pb.finish_and_clear();
    Ok(entry.context("package not found")?.dependency.clone())
  }
}