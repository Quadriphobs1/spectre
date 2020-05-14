use crate::Driver;
use crate::Result;

#[derive(Clone, Debug)]
pub struct MySQLDriver {}

impl MySQLDriver {
  pub fn new() -> Self {
    unimplemented!()
  }
}

impl Driver for MySQLDriver {
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
