use std::rc::Rc;
use crate::core;

pub struct Puff
{
  pub config: Rc<core::Config>,
  pub args: Rc<core::Args>,
}

impl Puff
{
  pub fn new(config: Rc<core::Config>, args: Rc<core::Args>) -> Self
  {
    Self
    {
      config,
      args
    }
  }

  pub fn install(&mut self) -> anyhow::Result<&mut Self>
  {
    Ok(self)
  }

  pub fn build(&mut self) -> anyhow::Result<&mut Self>
  {
    Ok(self)
  }
}