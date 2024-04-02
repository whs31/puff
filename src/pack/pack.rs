use std::collections::HashMap;
use std::fs::File;
use std::time::Duration;
use anyhow::Context;
use colored::Colorize;
use flate2::write::GzEncoder;
use indicatif::ProgressBar;
use crate::builder::Recipe;
use crate::manifest::Manifest;
use crate::names::PACKED_SOURCE_TARBALL_NAME;
use crate::types::{Arch, Distribution, OperatingSystem, VersionRange};

pub fn pack(source: &str, target: &str) -> anyhow::Result<()> {
  let tar_gz = File::create(target)?;
  let enc = GzEncoder::new(tar_gz, flate2::Compression::default());
  let mut tar = tar::Builder::new(enc);

  for entry in std::fs::read_dir(source)? {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir() {
      let dirname = path.file_name().unwrap().to_os_string().into_string().unwrap();
      if dirname != ".git"
        && !dirname.contains("build")
        && !dirname.contains("target")
        && dirname != ".idea"
        && dirname != target
      {
        tar.append_dir_all(dirname, path.clone())?;
      }
    }

    if path.is_file() {
      let filename = path.file_name().unwrap().to_os_string().into_string().unwrap();
      if filename != target
        && !filename.ends_with(".user")
      {
        tar.append_path_with_name(path.clone(), filename)?;
      }
    }
  }
  tar.finish()?;
  Ok(())
}

pub fn pack_with_manifest(path: &str) -> anyhow::Result<String>
{
  let manifest = Manifest::from_directory(path)?;
  let mut fmt: HashMap<String, String> = HashMap::new();
  fmt.insert("name".to_string(), manifest.this.name.clone());
  fmt.insert("version".to_string(), manifest.this.version.clone().to_string());
  let tar_name = strfmt::strfmt(PACKED_SOURCE_TARBALL_NAME, &fmt)
    .context("failed to format tarball name")?;
  pack_into_data_folder(path, &tar_name, &manifest.this.name, manifest.this.version)
}

pub fn pack_for_cache(path: &str, arch: Arch, distribution: Distribution, os: OperatingSystem) -> anyhow::Result<String>
{
  let manifest = Manifest::from_directory(path)?;
  let tar_name = format!("{}-{}-{}-{}-{}.tar.gz",
    &manifest.this.name,
    &manifest.this.version,
    arch.to_string(),
    os.to_string(),
    distribution.to_string()
  );
  pack_into_data_folder(path, &tar_name, &manifest.this.name, manifest.this.version)
}

fn pack_into_data_folder(path: &str, tar_name: &str, name: &str, version: VersionRange) -> anyhow::Result<String>
{
  let pb = ProgressBar::new_spinner();
  pb.enable_steady_tick(Duration::from_millis(100));
  pb.set_message(format!("packing {}@{}",
                         name.to_string().bold().magenta(),
                         version.clone().to_string().bold().green()
  ));

  let _ = Recipe::from_directory(path)?; // only for checking for it's existence

  let cache_dir = directories::ProjectDirs::from("io", crate::names::NAME, crate::names::NAME)
    .context("failed to get project directories")?
    .data_dir()
    .join("packed");
  std::fs::create_dir_all(cache_dir.as_path())?;
  let target_path = cache_dir.join(tar_name);
  pack(path, target_path.to_str().unwrap())?;

  pb.finish_with_message(format!("{} {}@{}",
                                 "successfully packed".to_string().green().bold(),
                                 name.to_string().bold().magenta(),
                                 version.to_string().bold().green()
  ));

  Ok(target_path.into_os_string().into_string().unwrap())
}