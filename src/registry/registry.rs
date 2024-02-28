use crate::registry::entry::RegistryEntry;

pub struct Registry
{
  pub packages: Vec<RegistryEntry>,
  registry_url: String,
  registry_path: String
}

impl Registry
{
  pub fn new(url: &str, path: &str) -> Self
  {
    Self
    {
      packages: vec![],
      registry_url: String::from(url),
      registry_path: String::from(path)
    }
  }
}
