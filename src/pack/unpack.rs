pub fn unpack(from: &str, to: &str) -> anyhow::Result<()> {
  std::fs::create_dir_all(to)?;

  let tar_gz = std::fs::File::open(from)?;
  let tar = flate2::read::GzDecoder::new(tar_gz);
  let mut archive = tar::Archive::new(tar);
  archive.unpack(to)?;

  Ok(())
}

