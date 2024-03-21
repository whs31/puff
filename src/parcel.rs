use std::rc::Rc;
use crate::core;

pub struct Parcel
{
  pub config: Rc<core::Config>,
  pub args: Rc<core::Args>,
}

impl Parcel
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