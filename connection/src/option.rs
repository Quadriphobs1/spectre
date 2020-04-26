use crate::Datasource;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConnectionOption {
  #[serde(default)]
  pub name: String,
  #[serde(default)]
  pub logging: bool, // TODO: Should support different logging options such as all, query, error, schema, warn, info, log and accept a custom logger
  #[serde(default = "Datasource::default")]
  pub datasource: Datasource,
  #[serde(default)]
  pub auto_migrate: bool,
  // entities/models, migrations, subscribers
}

impl Default for ConnectionOption {
  fn default() -> Self {
    ConnectionOption {
      name: String::from("default"),
      logging: false,
      auto_migrate: false,
      datasource: Datasource::default(),
    }
  }
}

impl ConnectionOption {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn normalize() {
    unimplemented!()
  }
}
