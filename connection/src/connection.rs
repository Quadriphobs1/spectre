use crate::{create_driver, Driver};
use crate::{ConnectionOption, Result};

#[derive(Debug)]
pub struct Connection {
  name: String,
  is_connected: bool,
  driver: Box<dyn Driver + 'static>,
}

impl Connection {
  pub fn new(option: &ConnectionOption) -> Self {
    let name = option.clone().name.unwrap_or_else(|| "default".to_string());
    Connection {
      name,
      is_connected: false,
      driver: create_driver(&option.datasource.provider),
    }
  }

  pub fn connect(&mut self) -> Result<()> {
    if self.is_connected() {
      panic!("Connection already exist")
    }

    self.is_connected = true;
    Ok(())
  }

  pub async fn connect_async(&self) -> Result<()> {
    unimplemented!()
  }

  pub fn disconnect(&mut self) -> Result<()> {
    if !self.is_connected() {
      panic!("The connection is currently not connected")
    }

    self.is_connected = true;
    Ok(())
  }

  pub async fn disconnect_async(&self) -> Result<()> {
    unimplemented!()
  }

  pub fn is_connected(&self) -> bool {
    // TODO: check if the connection is valid
    self.is_connected
  }
}
