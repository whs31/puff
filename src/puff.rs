use std::rc::Rc;
use crate::builder::{Builder, Recipe};
use crate::core;
use crate::core::args::Command;
use crate::manifest::Manifest;

pub struct Puff
{
  pub config: Rc<core::Config>,
  pub args: Rc<core::Args>,
  pub env: Rc<core::Environment>
}

impl Puff
{
  pub fn new(config: Rc<core::Config>, args: Rc<core::Args>, env: Rc<core::Environment>) -> Self
  {
    Self
    {
      config,
      args,
      env
    }
  }

  pub fn install(&mut self) -> anyhow::Result<&mut Self>
  {
    println!("{}", self.env.pretty_print());
    Ok(self)
  }

  pub fn build(&mut self) -> anyhow::Result<&mut Self>
  {
    let path = match &self.args.command {
      Some(command) => match command {
        Command::Build(x) => {
          match &x.folder {
            Some(y) => y.clone(),
            None => std::env::current_dir()?.into_os_string().into_string().unwrap(),
          }
        }
        _ => return Ok(self),
      }
      None => return Ok(self),
    };
    self.build_packet(&path)?;
    Ok(self)
  }

  fn build_packet(&self, path: &str) -> anyhow::Result<()>
  {
    let manifest = Manifest::from_directory(path)?;
    let recipe = Recipe::from_directory(path)?;
    let builder = Builder::new(self.config.clone(), self.env.clone());
    builder.build(&recipe, path)?;
    Ok(())
  }
}