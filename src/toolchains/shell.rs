use std::path::PathBuf;
use std::path::Path;
use anyhow::{Context};
use shlex::Shlex;
use crate::builder::Recipe;
use crate::toolchains::Toolchain;
use crate::types::Distribution;

pub struct ShellToolchain;

impl ShellToolchain
{
  pub fn new() -> Self { Self }
}

impl Toolchain for ShellToolchain
{
  fn build_from_recipe(&self, recipe: &Recipe, source_directory: &str, distribution: Distribution) -> anyhow::Result<PathBuf>
  {
    let toolchain = recipe
      .extract_toolchain(distribution)?
      .shell
      .context("shell toolchain was requested to build package but recipe is not configured for shell")?;

    for cmd in &toolchain {
      let shell_args = Shlex::new(cmd).collect::<Vec<_>>();
      let mut command = std::process::Command::new(shell_args.get(0).context("invalid shell command")?);
      command.args(&shell_args[1..]);
      command.current_dir(source_directory);
      command
        .output()
        .context(format!("failed to execute shell command ({}), output: {:?}", cmd, command.output()))?;
    }

    let target = Path::new(source_directory)
      .join(crate::names::TARGET_FOLDER)
      .join(crate::names::EXPORT_FOLDER);
    crate::toolchains::utl::copy_package_metafiles(
      source_directory,
      target
        .to_str()
        .context("failed to convert target directory path to string")?
    )?;

    Ok(target)
  }
}