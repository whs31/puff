use crate::builder::Recipe;
use crate::types::Distribution;

pub trait Toolchain
{
  fn build_from_recipe(&self, recipe: &Recipe, source_directory: &str, distribution: Distribution) -> anyhow::Result<()>;
}