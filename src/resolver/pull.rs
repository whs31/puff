use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, ensure};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use log::{debug, trace, warn};
use futures_util::stream::StreamExt;
use crate::resolver::Dependency;

#[tokio::main]
pub async fn pull_from_artifactory(
  dep: &Dependency,
  artifactory_base_url: &str,
  artifactory_api_url: &str,
  username: &str,
  token: &str
) -> anyhow::Result<Vec<u8>> {
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
  let url = strfmt::strfmt(artifactory_base_url, &fmt)
    .context("failed to format artifactory url")?;
  let api_url = strfmt::strfmt(artifactory_api_url, &fmt)
    .context("failed to format artifactory api url")?;

  trace!("url: {}", url);
  trace!("api url: {}", api_url);

  let result = client
    .get(url.as_str())
    .basic_auth(username, Some(token))
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
    .basic_auth(username, Some(token))
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
  //ensure!(md5_from_api == md5.hexdigest(), "md5 checksum mismatch");
  Ok(data)
}

pub fn save_to(path: &str, data: &[u8]) -> anyhow::Result<()> {
  trace!("saving {} bytes to {}", data.len(), path);
  std::fs::create_dir_all(Path::new(path).parent().unwrap())?;
  std::fs::write(path, data)?;
  Ok(())
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
  // unpack tar.gz
  // strip prefix
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