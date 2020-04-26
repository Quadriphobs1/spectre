extern crate spectre_connection;

use spectre_connection::Result;

#[derive(Default, Debug)]
pub struct Client {}

impl Client {
  pub fn new() -> Self {
    Self::default()
  }
}
pub fn establish_connection() -> Result<()> {
  Ok(())
}

pub fn establish_connections() -> Result<()> {
  Ok(())
}
