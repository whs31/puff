use std::fmt::Display;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Distribution
{
  Static,
  Shared,
  Sources,
  Unknown
}

impl Default for Distribution
{
  fn default() -> Self
  {
    Self::Unknown
  }
}

impl From<&str> for Distribution
{
  fn from(value: &str) -> Self
  {
    match value
    {
      "static" | "static-lib" => Self::Static,
      "shared" | "dynamic" | "dynamic-lib" => Self::Shared,
      "sources" | "src" | "source" => Self::Sources,
      _ => Self::Unknown
    }
  }
}

impl From<String> for Distribution
{
  fn from(value: String) -> Self
  {
    Self::from(value.as_str())
  }
}

impl Display for Distribution
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{}", match self
    {
      Self::Static => "static",
      Self::Shared => "shared",
      Self::Sources => "sources",
      _ => "unknown"
    })
  }
}