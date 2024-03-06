pub fn locate_poppy() -> anyhow::Result<String>
{
  let mut path = std::env::current_exe()?;
  path.pop();
  Ok(path.to_str().unwrap().to_string())
}