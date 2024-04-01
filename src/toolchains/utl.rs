use std::{fs, io};
use std::path::Path;

pub fn copy_package_metafiles(source_directory: &str, target_directory: &str) -> anyhow::Result<()>
{
  let source_folder = Path::new(source_directory);
  let target_folder = Path::new(target_directory);
  fs::create_dir_all(target_folder)?;
  match copy_dir_all(source_folder.join(".puff"), target_folder.join(".puff"))
  {
    Ok(_) => (),
    Err(e) => return Err(anyhow::anyhow!("failed to copy .puff directory from {} to {} ({})",
      source_folder.join(".puff").display(),
      target_folder.join(".puff").display(),
      e
    )),
  }
  match fs::copy(source_folder.join("Puff.toml"), target_folder.join("Puff.toml"))
  {
    Ok(_) => (),
    Err(e) => return Err(anyhow::anyhow!("failed to copy Puff.toml from {} to {} ({})",
      source_folder.join("Puff.toml").display(),
      target_folder.join("Puff.toml").display(),
      e
    )),
  }
  Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
  fs::create_dir_all(&dst)?;
  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let ty = entry.file_type()?;
    if ty.is_dir() {
      copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
    } else {
      fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
    }
  }
  Ok(())
}