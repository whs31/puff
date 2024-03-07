use std::collections::HashMap;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use crate::resolver::Dependency;
use crate::utils::helper_types::{Distribution, PlatformArch, Version};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryEntry
{
  pub name: String,
  pub versions: HashMap<Version, HashMap<Distribution, Vec<PlatformArch>>>
}

impl RegistryEntry
{
  pub fn pretty_format(&self) -> String
  {
    let mut str_t = format!("{:<12}: ", self.name.yellow().bold());
    let mut flag = false;
    let largest_version = self.versions
      .keys()
      .max();
    for (v, dist) in &self.versions
    {
      if !flag {
        if v != largest_version.unwrap() {
          continue;
        }
        str_t = format!("{} {:<8}",
          str_t,
          v.to_string().green().bold()
        );
        flag = true;
      } else {
        continue;
      }

      for (d, platforms) in dist
      {
        str_t = format!("{} {}",
          str_t,
          d.to_string().cyan().bold()
        );
        for platform in platforms
        {
          str_t = format!("{}/{}",
            str_t,
            platform.to_short_string().blue().dimmed()
          );
        }
        str_t = format!("{} ", str_t);
      }
    }
    if self.versions.len() > 1 {
      str_t = format!("{}and {} other versions",
        str_t,
        format!("{}", self.versions.len() - 1).magenta().bold()
      );
    }
    str_t
  }

  pub fn into_dependency(&self) -> anyhow::Result<Vec<Dependency>>
  {
    let mut res: Vec<Dependency> = Vec::new();

    for (v, dist) in &self.versions
    {
      for (d, platforms) in dist
      {
        for platform in platforms
        {
          res.push(Dependency {
            name: self.name.clone(),
            version: v.clone(),
            distribution: d.clone(),
            arch: *platform
          });
        }
      }
    }
    Ok(res)
  }
}