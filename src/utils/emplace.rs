use std::path::Path;
use log::trace;

pub fn add_gitignore(to: &str) -> anyhow::Result<()>
{
  let contents = "*\n!.gitignore";
  std::fs::write(Path::new(to).join(".gitignore"), contents)?;
  trace!("added .gitignore to {}", to);
  Ok(())
}