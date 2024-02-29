use log::trace;

pub fn push_to_artifactory(url: &str, data: &[u8], username: &str, token: &str) -> anyhow::Result<()> {
  trace!("pushing {} bytes to {}", data.len(), url);
  trace!("username: {}", username);
  trace!("token: {}", token);
  let client = reqwest::blocking::Client::builder()
    //.redirect(reqwest::redirect::Policy::none())
    .build()?;
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

