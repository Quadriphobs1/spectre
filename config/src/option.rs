use crate::{Logger, Logging};
use connection::ConnectionOption;

#[derive(Deserialize, Serialize, Debug)]
pub struct CliOption {
  // Directory where migrations should be created by default.
  #[serde(default)]
  migration_dir: Option<String>,
}

impl Default for CliOption {
  fn default() -> Self {
    CliOption {
      migration_dir: Some("/migrations".to_string()),
    }
  }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConfigOption {
  #[serde(default)]
  pub logging: Option<Logging>,
  #[serde(default)]
  pub version: String,
  #[serde(default)]
  pub connections: Vec<ConnectionOption>,
  #[serde(default)]
  pub cli: Option<CliOption>,
  #[serde(default)]
  pub logger: Option<Logger>,
}

impl Default for ConfigOption {
  fn default() -> Self {
    let mut connections = Vec::new();
    connections.push(ConnectionOption::new());
    ConfigOption {
      // TODO: This should not be hardcoded
      version: String::from("1.0"),
      cli: Some(CliOption::default()),
      logging: Some(Logging::Error),
      logger: Some(Logger::default()),
      connections,
    }
  }
}

impl ConfigOption {
  pub fn new() -> Self {
    Self::default()
  }
}
