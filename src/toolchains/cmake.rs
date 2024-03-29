use std::env::temp_dir;
use std::path::PathBuf;
use anyhow::{bail, Context, ensure};
use crate::builder::Recipe;
use crate::toolchains::Toolchain;
use crate::types::Distribution;

pub struct CMakeToolchain
{
  pub configure_command: String,
  pub configure_additional_arguments: Vec<String>,
}

impl CMakeToolchain
{
  pub fn new(config: &crate::core::Config) -> Self
  {
    Self
    {
      configure_command: config.toolchain.cmake.configure_command.clone(),
      configure_additional_arguments: config.toolchain.cmake.configure_additional_definitions
        .iter()
        .map(|x| format!("{}={}", x.0, x.1))
        .collect()
    }
  }
}

impl Toolchain for CMakeToolchain
{
  fn build_from_recipe(&self, recipe: &Recipe, source_directory: &str, distribution: Distribution) -> anyhow::Result<PathBuf>
  {
    let target_temp = temp_dir()
      .join(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos().to_string())
      .join(crate::names::TARGET_FOLDER);
    let mut command = std::process::Command::new(self.configure_command.clone());
    command
      .arg("-S")
      .arg(source_directory)
      .arg("-B")
      .arg(target_temp.clone());
    for x in &self.configure_additional_arguments {
      command.arg("-D").arg(x);
    }
    command.arg(format!("-DCMAKE_PREFIX_PATH={}", crate::names::DEPENDENCIES_FOLDER));

    let toolchain = recipe
      .extract_toolchain(distribution)?
      .cmake
      .context("cmake toolchain was requested to build package but recipe is not configured for cmake")?;
    // generator
    if let Some(generator) = toolchain.generator {
      command.arg(format!("-G{}", generator.as_str()));
    }
    if let Some(definitions) = toolchain.definitions.as_ref() {
      for x in definitions {
        command.arg(format!("-D{}={}", x.0.to_uppercase(), x.1.to_uppercase()));
      }
    }
    command.stdout(std::process::Stdio::null());
    ensure!(command.status()?.success(), "cmake configure step failed");

    let mut command = std::process::Command::new("cmake");
    command
      .arg("--build")
      .arg(target_temp.clone())
      .arg("--config")
      .arg("release")
      .arg("--parallel");
    command.stdout(std::process::Stdio::null());
    ensure!(command.status()?.success(), "cmake build step failed");

    let export_folder = target_temp
      .clone()
      .join(crate::names::EXPORT_FOLDER);
    let mut command = std::process::Command::new("cmake");
    command
      .arg("--install")
      .arg(target_temp.clone())
      .arg("--prefix")
      .arg(export_folder.clone());
    command.stdout(std::process::Stdio::null());
    ensure!(command.status()?.success(), "cmake install step failed");

    crate::toolchains::utl::copy_package_metafiles(
      source_directory,
      export_folder
        .to_str()
        .context("failed to convert export directory path to string")?
    )?;

    Ok(export_folder)
  }
}