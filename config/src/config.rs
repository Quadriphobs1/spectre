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
pub struct Config {
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

impl Default for Config {
  fn default() -> Self {
    let mut connections = Vec::new();
    connections.push(ConnectionOption::new());
    Config {
      // TODO: This should not be hardcoded
      version: String::from("1.0"),
      cli: Some(CliOption::default()),
      logging: Some(Logging::Error),
      logger: Some(Logger::default()),
      connections,
    }
  }
}

impl Config {
  pub fn new() -> Self {
    Self::default()
  }

  #[inline(always)]
  pub fn logging(mut self, logging: Logging) -> Self {
    self.logging = Some(logging);
    self
  }

  #[inline(always)]
  pub fn version(mut self, version: &str) -> Self {
    self.version = version.into();
    self
  }

  #[inline(always)]
  pub fn cli(mut self, migration_dir: &str) -> Self {
    self.cli = Some(CliOption {
      migration_dir: Some(migration_dir.into()),
    });
    self
  }

  #[inline(always)]
  pub fn logger(mut self, logger: Logger) -> Self {
    self.logger = Some(logger);
    self
  }

  #[inline(always)]
  pub fn connections(mut self, connections: &[ConnectionOption]) -> Self {
    self.connections = Vec::from(connections);
    self
  }
}
