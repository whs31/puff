use log::{debug, info, trace, warn};

pub fn push_to_artifactory(
  url: &str,
  data: &[u8],
  username: &str,
  token: &str,
  force: bool
) -> anyhow::Result<()> {
  if !force {
    trace!("no force push available");
  } else {
    warn!("force pushing. this can be dangerous!");
  }
  trace!("pushing {} bytes to {}", data.len(), url);
  trace!("username: {}", username);
  trace!("token: {}", token);
  let client = reqwest::blocking::Client::builder()
    //.redirect(reqwest::redirect::Policy::none())
    .build()?;

  let exists = client
    .get(url)
    .basic_auth(username, Some(token))
    .send()?;

  if exists.status() != 404 {
    info!("file already exists on artifactory");
    if !force {
      debug!("skipping push");
      return Ok(());
    }
    else {
      warn!("OVERRIDING PREVIOUS PACKAGE ON ARTIFACTORY!");
    }
  }
  let res = client
    .put(url)
    .basic_auth(username, Some(token))
    .body(data.to_vec())
    .send()?;
  trace!("status: {}", res.status());
  Ok(())
}

pub fn tar_to_binary(path: &str) -> anyhow::Result<Vec<u8>> {
  let data = std::fs::read(path)?;
  Ok(data)
}

