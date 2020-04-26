use crate::ConfigOption;
use crate::Result;

pub struct Parser;

impl Parser {
  /// parse the config file and replace any env variables with their respect values
  pub fn into(bytes: &[u8]) -> Result<ConfigOption> {
    let config_option = toml::from_slice::<ConfigOption>(&bytes).unwrap();
    Ok(config_option)
  }

  pub fn from(option: &ConfigOption) -> Result<Vec<u8>> {
    let config = toml::to_string(&option).unwrap();
    Ok(config.into_bytes())
  }
}
