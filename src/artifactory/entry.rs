use crate::resolver::Dependency;

#[derive(Debug, Clone)]
pub struct Entry
{
  pub dependency: Dependency,
  pub url: String
}