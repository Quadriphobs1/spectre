use spectre_connection::ConnectionOption;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConfigOption {
  #[serde(default)]
  pub version: String,
  #[serde(default)]
  pub connections: Vec<ConnectionOption>,
}

impl Default for ConfigOption {
  fn default() -> Self {
    let mut connections = Vec::new();
    connections.push(ConnectionOption::new());
    ConfigOption {
      // TODO: This should not be hardcoded
      version: String::from("1.0"),
      connections,
    }
  }
}

impl ConfigOption {
  pub fn new() -> Self {
    Self::default()
  }
}
