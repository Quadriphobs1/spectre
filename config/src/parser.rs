use crate::ConfigOption;
use crate::Result;

pub struct Parser;

impl Parser {
  /// parse the config file and replace any env variables with their respect values
  pub fn into(bytes: &[u8]) -> Result<ConfigOption> {
    let config_option = serde_yaml::from_slice::<ConfigOption>(&bytes)?;
    Ok(config_option)
  }

  pub fn from(option: &ConfigOption) -> Result<Vec<u8>> {
    let config = serde_yaml::to_string(&option)?;
    Ok(config.into_bytes())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ConfigOption;
  use indoc::indoc;
  use spectre_connection::ConnectionOption;

  #[test]
  fn parse_from_default() {
    let option = ConfigOption::new();

    let result = Parser::from(&option);
    assert!(result.is_ok())
  }

  #[test]
  fn parse_from() {
    let option = ConfigOption {
      version: String::from("1.0"),
      connections: vec![
        ConnectionOption::new(),
        ConnectionOption::new(),
        ConnectionOption::new(),
      ],
    };

    let result = Parser::from(&option);
    assert!(result.is_ok())
  }

  #[test]
  fn parse_into() {
    let option = indoc!(
      r#"
        version: "1.0"
        connections:
          - name: "default"
            datasource:
              url: postgresql://postgres:admin@localhost:5432/postgres?schema=public"
              provider: "postgres"
            auto_migrate: false
            logging: false
      "#
    );
    let result = Parser::into(option.as_bytes());
    assert!(result.is_ok())
  }

  #[test]
  fn parse_into_error() {
    let option = indoc!(
      r#"
        version: "1.0"
        connections:
      "#
    );
    let result = Parser::into(option.as_bytes());
    assert!(result.is_err());
  }
}
