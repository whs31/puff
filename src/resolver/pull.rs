use std::collections::HashMap;
use anyhow::{Context, ensure};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use log::trace;
use futures_util::stream::StreamExt;
use crate::resolver::Dependency;

#[tokio::main]
pub async fn pull_from_artifactory(dep: &Dependency, artifactory_base_url: &str, username: &str, token: &str) -> anyhow::Result<Vec<u8>> {
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

  let result = client
    .get(url.as_str())
    .basic_auth(username, Some(token))
    .send()
    .await?;
  trace!("response status: {}", result.status());
  ensure!(result.status().is_success(), "pulling from artifactory failed with status code {}", result.status().as_str());
  let total = result
    .content_length()
    .unwrap_or(1);
  trace!("total bytes: {}", total);
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
  // todo: parse json and compare md5
  Ok(data)
}