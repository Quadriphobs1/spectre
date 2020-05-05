use crate::Datasource;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConnectionOption {
  #[serde(default)]
  pub name: Option<String>,
  #[serde(default)]
  pub datasource: Datasource,
  pub auto_migrate: Option<bool>,
  #[serde(default)]
  pub logging: Option<bool>,
  // entities/models, migrations, subscribers
}

impl Default for ConnectionOption {
  fn default() -> Self {
    ConnectionOption {
      name: Some(String::from("default")),
      logging: Some(false),
      auto_migrate: Some(false),
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
