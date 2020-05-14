use crate::{Connection, ConnectionOption, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ConnectionManager {
  connections: HashMap<String, Connection>,
}

impl Default for ConnectionManager {
  fn default() -> Self {
    ConnectionManager {
      connections: HashMap::new(),
    }
  }
}

impl ConnectionManager {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn connections(&mut self) -> &HashMap<String, Connection> {
    &self.connections
  }

  pub fn has(&self, name: &str) -> bool {
    self.connections.contains_key(&name.to_string())
  }

  pub fn get(&self, name: &str) -> Option<&'_ Connection> {
    self.connections.get(&name.to_string())
  }

  // Check the size of connections initialized
  pub fn size(&self) -> usize {
    self.connections.len()
  }

  pub fn create(&mut self, option: &ConnectionOption) -> Result<()> {
    let name = option.clone().name.unwrap_or_else(|| "default".to_string());
    // check if the connection is already registered
    let existing = self.has(&name);

    if existing {
      // TODO: should log into the console only
      panic!("Connection exist already")
    }

    let connection = Connection::new(option);
    self.connections.insert(name, connection);

    Ok(())
  }

  // remove a connection irrespective of whether it is disconnected
  pub fn remove(&mut self, name: &str) -> Result<Connection> {
    match self.connections.remove(name) {
      Some(connection) => Ok(connection),
      None => panic!("Error connection does not exist"),
    }
  }

  // remove all disconnected connection and clean up the connection map
  pub fn cleanup(&mut self) -> Result<()> {
    self
      .connections
      .retain(|key, connection| connection.is_connected() && key != &"default".to_string());

    Ok(())
  }

  // add fn
}
