use crate::{ConfigOption, Result};

macro_rules! regex {
  ($re:expr) => {
    ::regex::Regex::new($re).unwrap()
  };
}

/// parse the config file and replace any env variables with their respect values
pub fn into(bytes: &[u8]) -> Result<ConfigOption> {
  let config_option = serde_yaml::from_slice::<ConfigOption>(&bytes)?;
  Ok(config_option)
}

pub fn from(option: &ConfigOption) -> Result<Vec<u8>> {
  let config = serde_yaml::to_string(&option)?;
  Ok(config.into_bytes())
}

pub fn transform_from_env(template: &mut String) -> Result<String> {
  let mut content = String::new();
  let mut last = 0;
  let re = regex!(r"\B\$\{([A-Z0-9_]*?)\}");

  for cap in re.captures_iter(&template) {
    let range = cap.get(0).unwrap();
    content.push_str(&template[last..range.start()]);
    let key = cap.get(1).unwrap().as_str();
    ::std::env::var(key).map(|val| {
      content.push_str(&val);
    })?;
    last = range.end();
  }

  if content.is_empty() && !template.is_empty() {
    content = (*template).to_string();
  }

  Ok(content)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ConfigOption;
  use connection::ConnectionOption;
  use indoc::indoc;
  use std::env;

  #[test]
  fn transform_example_var() {
    let expected = "some-example-var";
    env::set_var("EXAMPLE_VAR", expected);
    let mut text = String::from("${EXAMPLE_VAR}");
    let res = transform_from_env(&mut text);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "some-example-var");
  }

  #[test]
  fn transform_var_with_number() {
    let expected = "some-example-var";
    env::set_var("EXAMPLE_123_VAR", expected);
    let mut text = String::from("${EXAMPLE_123_VAR}");
    let res = transform_from_env(&mut text);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "some-example-var");
  }

  #[test]
  fn does_not_transform_invalid_example_var() {
    let mut text = String::from("${EXAMPLE VAR}");
    let res = transform_from_env(&mut text);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "${EXAMPLE VAR}");
  }

  #[test]
  fn does_not_transform_special_char_var() {
    let mut text = String::from("${SPECIAL@CHARACTER}");
    let res = transform_from_env(&mut text);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "${SPECIAL@CHARACTER}");
  }

  #[test]
  fn does_not_transform_small_caps_var() {
    let mut text = String::from("${smallcaps}");
    let res = transform_from_env(&mut text);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "${smallcaps}");
  }

  #[test]
  fn does_not_transform_char_in_between_var() {
    let mut text = String::from("${CHAR${INBETWEEN}}");
    let res = transform_from_env(&mut text);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "${CHAR${INBETWEEN}}");
  }

  #[test]
  fn does_not_transform_multiple_invalid_example_var() {
    let mut text = String::from("example \n ${EXAMPLE VAR} \n ${ANOTHER VAR}");
    let res = transform_from_env(&mut text);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "example \n ${EXAMPLE VAR} \n ${ANOTHER VAR}");
  }

  #[test]
  fn transform_repeated_example_var() {
    env::set_var("EXAMPLE_VAR", "some-example-var");
    env::set_var("ANOTHER_EXAMPLE_VAR", "another-example-var");
    let mut text = String::from("${EXAMPLE_VAR}${ANOTHER_EXAMPLE_VAR}");
    let res = transform_from_env(&mut text);
    assert!(res.is_ok());
    assert_eq!(
      res.unwrap(),
      String::from("some-example-varanother-example-var")
    );
  }

  #[test]
  fn transform_special_example_var() {
    env::set_var("EXAMPLE_VAR", "some-example-var");
    env::set_var("ANOTHER_EXAMPLE_VAR", "another-example-var");
    let mut text = String::from("${EXAMPLE_VAR} ${ANOTHER_EXAMPLE_VAR}");
    let res = transform_from_env(&mut text);
    assert!(res.is_ok());
    assert_eq!(
      res.unwrap(),
      String::from("some-example-var another-example-var")
    );
  }

  #[test]
  fn transform_new_line_example_var() {
    env::set_var("EXAMPLE_VAR", "some-example-var");
    env::set_var("ANOTHER_EXAMPLE_VAR", "another-example-var");
    let mut text = String::from("${EXAMPLE_VAR} \n ${ANOTHER_EXAMPLE_VAR}");
    let res = transform_from_env(&mut text);
    assert!(res.is_ok());
    assert_eq!(
      res.unwrap(),
      String::from("some-example-var \n another-example-var")
    );
  }

  #[test]
  fn does_not_transform_var() {
    let mut text = String::from("${NEW_DATABASE_URL}");
    let res = transform_from_env(&mut text);
    assert!(res.is_err());
  }

  #[test]
  fn does_not_transform_plain_text() {
    let expected = indoc!(
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
    let res = transform_from_env(&mut expected.to_string());
    assert!(res.is_ok());
    assert_eq!(expected, res.unwrap());
  }

  #[test]
  fn transform_env_text() {
    env::set_var("DATABASE_URL", "database-url");
    let mut content = indoc!(
      r#"
        version: "1.0"
        connections:
          - name: "default"
            datasource:
              url: ${DATABASE_URL}
              provider: "postgres"
            auto_migrate: false
            logging: false
      "#
    )
    .to_string();
    let res = transform_from_env(&mut content);
    assert!(&res.is_ok());
    assert!(&res.unwrap().contains("database-url"));
  }

  #[test]
  fn transform_many_env_text() {
    env::set_var("DATABASE_URL", "database-url");
    env::set_var("ANOTHER_DATABASE_URL", "another-database-url");
    let mut content = indoc!(
      r#"
        version: "1.0"
        connections:
          - name: "default"
            datasource:
              url: ${DATABASE_URL}
              provider: "postgres"
            auto_migrate: false
            logging: false
          - name: "default"
            datasource:
              url: ${ANOTHER_DATABASE_URL}
              provider: "postgres"
            auto_migrate: false
            logging: false
      "#
    )
    .to_string();
    let res = transform_from_env(&mut content);
    assert!(res.is_ok());
    assert!(res.unwrap().contains("another-database-url"));
  }

  #[test]
  fn parse_from_default() {
    let option = ConfigOption::new();

    let result = from(&option);
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
      logging: None,
      cli: None,
      logger: None,
    };

    let result = from(&option);
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
    let result = into(option.as_bytes());
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
    let result = into(option.as_bytes());
    assert!(result.is_err());
  }
}
