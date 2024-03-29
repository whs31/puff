pub fn copy_package_metafiles(source_directory: &str, target_directory: &str) -> anyhow::Result<()>
{
  let source = std::path::Path::new(source_directory);
  let target = std::path::Path::new(target_directory);
  std::fs::copy(source.join(".puff"), target.join(".puff"))?;
  std::fs::copy(source.join("Puff.toml"), target.join("Puff.toml"))?;
  Ok(())
}