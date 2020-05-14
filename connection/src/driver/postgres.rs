use crate::Driver;
use crate::Result;

#[derive(Clone, Debug)]
pub struct PotsgresDriver {}

impl PotsgresDriver {
  pub fn new() -> Self {
    unimplemented!()
  }
}

impl Driver for PotsgresDriver {
  fn connect(&self) -> Result<()> {
    Ok(())
  }

  fn disconnect(&self) -> Result<()> {
    Ok(())
  }

  fn post_connect(&self) -> Result<()> {
    Ok(())
  }

  fn establish(database_url: &str) -> Result<Self>
  where
    Self: Sized,
  {
    unimplemented!()
  }
}
