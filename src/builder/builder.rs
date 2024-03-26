use std::rc::Rc;
use crate::core;

pub struct Builder
{
  pub config: Rc<core::Config>,
  pub env: Rc<core::Environment>
}

impl Builder
{
  pub fn new(config: Rc<core::Config>, env: Rc<core::Environment>) -> Self
  {
    Self
    {
      config,
      env
    }
  }

  pub fn build(&self, recipe: &crate::builder::Recipe, path: &str) -> anyhow::Result<&Self>
  {
    println!("building {}...", path);

    Ok(self)
  }
}