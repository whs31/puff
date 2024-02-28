use crate::resolver::Dependency;

pub struct DependencyStack
{
  stack: Vec<Dependency>
}

impl DependencyStack
{
  pub fn new() -> Self
  {
    Self
    {
      stack: Vec::new()
    }
  }

  // todo: maybe push manifest as whole?
  // todo: also maybe hide push/pop from user?

  pub fn push(&mut self, dependency: Dependency) -> anyhow::Result<&mut Self>
  {
    // if self.check(&dependency) todo
    self.stack.push(dependency);
    Ok(self)
  }

  pub fn pop(&mut self) -> Option<Dependency> { self.stack.pop() }
  pub fn len(&self) -> usize { self.stack.len() }
  pub fn is_empty(&self) -> bool  { self.stack.is_empty() }

  pub fn check(&self, dependency: &Dependency) -> bool
  {
    todo!()
  }
}