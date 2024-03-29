use std::fs::File;
use flate2::write::GzEncoder;

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