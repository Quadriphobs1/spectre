// what to log, error by default
#[derive(Deserialize, Serialize, Debug)]
pub enum Logging {
  All,
  Query,
  Error,
  Warn,
  Info,
  Migration,
  Many(Vec<Logging>),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum LogMode {
  Advance,
  Simple,
  File,
  Debug,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Logger {
  // using the default logger provided by the crate
  #[serde(default)]
  mode: Option<LogMode>,
  // TODO: should accept a logger module
}

impl Default for Logger {
  fn default() -> Self {
    Logger {
      mode: Some(LogMode::Simple),
    }
  }
}
