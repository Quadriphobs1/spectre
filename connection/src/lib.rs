#[macro_use]
extern crate serde_derive;

mod datasource;
mod errors;
mod option;

pub use self::datasource::{Datasource, Provider};
pub use self::option::ConnectionOption;
use crate::errors::Error;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Default, Debug)]
pub struct Driver {}

#[derive(Default, Debug)]
pub struct Connection {
  name: String,
  is_connected: bool,
  option: ConnectionOption,
  driver: Driver,
}

impl Connection {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn connect() -> Result<Self> {
    unimplemented!()
  }

  pub async fn connect_async() -> Result<Self> {
    unimplemented!()
  }

  pub fn disconnect() -> Result<()> {
    Ok(())
  }

  pub async fn disconnect_async() -> Result<()> {
    Ok(())
  }
}

#[derive(Default, Debug)]
pub struct ConnectionManager {
  connections: Vec<Connection>,
}

impl ConnectionManager {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn has(_name: String) -> bool {
    false
  }

  pub fn get(_name: String) -> Result<Connection> {
    unimplemented!()
  }

  pub fn create(_option: ConnectionOption) -> Result<Connection> {
    unimplemented!()
  }
}
