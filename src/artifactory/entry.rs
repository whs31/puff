use std::collections::HashMap;
use anyhow::Context;
use crate::resolver::Dependency;

#[derive(Debug, Clone)]
pub struct Entry
{
  pub dependency: Dependency,
  pub url: String,
  pub api_url: String
}

impl Entry
{
  pub fn new(dependency: Dependency, fmt_url: &str, fmt_api_url: &str) -> anyhow::Result<Self>
  {
    let mut fmt: HashMap<String, String> = HashMap::new();
    fmt.insert("name".to_string(), dependency.name.clone());
    fmt.insert("version".to_string(), dependency.version.clone().to_string());
    fmt.insert("arch".to_string(), dependency.arch.to_string());
    fmt.insert("platform".to_string(), dependency.os.to_string());
    fmt.insert("dist".to_string(), dependency.distribution.to_string());

    let url = strfmt::strfmt(fmt_url, &fmt)
      .context("failed to format url")?;
    let api_url = strfmt::strfmt(fmt_api_url, &fmt)
      .context("failed to format api url")?;

    Ok(Self
    {
      dependency,
      url,
      api_url
    })
  }
}