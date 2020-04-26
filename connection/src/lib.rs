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
pub struct Connection {}

impl Connection {
  pub fn new() -> Self {
    Self::default()
  }
}

#[derive(Default, Debug)]
pub struct ConnectionManager {}

impl ConnectionManager {
  pub fn new() -> Self {
    Self::default()
  }
}

#[derive(Default, Debug)]
pub struct ConnectionOptionsReader {}

impl ConnectionOptionsReader {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn get(self, _name: &str) -> Result<ConnectionOption> {
    Ok(ConnectionOption::new())
  }
}
