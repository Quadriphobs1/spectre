use crate::Datasource;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
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

impl PartialEq for ConnectionOption {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

impl Eq for ConnectionOption {}

impl ConnectionOption {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn normalize() {
    unimplemented!()
  }
}

#[derive(Debug)]
pub struct ConnectionOptionManager {
  options: HashMap<String, ConnectionOption>,
}

impl ConnectionOptionManager {
  pub fn new(options: &[ConnectionOption]) -> Self {
    let mut options_map: HashMap<String, ConnectionOption> = HashMap::new();

    for option in options.iter() {
      let name = option.clone().name.unwrap_or_else(|| "default".into());
      options_map.insert(name.to_string(), option.clone());
    }

    ConnectionOptionManager {
      options: options_map,
    }
  }
  // Check if there is a connection
  pub fn has(&self, name: &str) -> bool {
    self.options.contains_key(&name.to_string())
  }

  // Get a connection option by name
  pub fn get(&self, name: &str) -> Option<&'_ ConnectionOption> {
    self.options.get(&name.to_string())
  }

  // Check how much connection option there is
  pub fn size(&self) -> usize {
    self.options.len()
  }

  // add fn
}
