use crate::ConfigOption;
use crate::Error;
use crate::Parser;
use crate::Result;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Config {
  config_name: String,
  config_bytes: Vec<u8>,
  extensions: Vec<String>,
  option: ConfigOption,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      config_name: String::from("spectre"),
      extensions: vec![String::from("yaml"), String::from("yml")],
      config_bytes: Vec::new(),
      option: ConfigOption::default(),
    }
  }
}

impl Config {
  /// Create a new config instance
  pub fn new() -> Self {
    Self::default()
  }

  /// Set the config file name
  pub fn config_name(&mut self, name: &str) {
    self.config_name = String::from(name);
  }

  /// Load the config file
  pub fn load(&mut self, root: &PathBuf) -> Result<()> {
    let path_base = Path::new(&root).join(&self.config_name);
    let path = self
      .extensions
      .iter()
      .find_map(|ext| is_in_path(&path_base, ext));

    if let Some(path) = path {
      let mut bytes = Vec::new();
      fs::File::open(&path)?.read_to_end(&mut bytes)?;
      self.config_bytes = bytes;
      let option = Parser::into(&self.config_bytes)?;
      // TODO: Substitute env variables from loaded values
      self.option = option;
      Ok(())
    } else {
      Err(Error::FileNotFound(root.to_path_buf()))
    }
  }

  pub fn create(&mut self, root: &PathBuf) -> Result<()> {
    use std::io::Write;

    let config = ConfigOption::default();
    let parsed_config = Parser::from(&config)?;
    // TODO: Should change the database url to use env syntax
    let path = root
      .join(&self.config_name)
      .with_extension(&self.extensions[0]);

    if path.exists() {
      return Err(Error::ConfigFileExist(self.config_name.to_string()));
    }

    let mut file = fs::File::create(path)?;
    file.write_all(&parsed_config)?;

    Ok(())
  }
}

pub fn is_in_path(path: &PathBuf, ext: &str) -> Option<PathBuf> {
  let file_path = path.with_extension(ext);

  if file_path.exists() && file_path.is_file() {
    return Some(file_path);
  }
  None
}

#[cfg(test)]
mod tests {
  extern crate tempfile;
  use self::tempfile::Builder;
  use super::{is_in_path, Config};
  use crate::Error;
  use indoc::indoc;
  use std::{env, fs};

  #[test]
  fn test_config_name() {
    let mut config = Config::new();
    assert_eq!(config.config_name, String::from("spectre"));
    config.config_name("example");
    assert_eq!(config.config_name, String::from("example"));
  }

  #[test]
  fn test_file_not_exist() {
    let dir = Builder::new().prefix("test-file").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let cargo_path = temp_path.join("Cargo.toml");

    fs::File::create(&cargo_path).unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    let err = config.load(&temp_path);

    assert!(err.is_err());

    assert_eq!(
      err.unwrap_err(),
      Error::FileNotFound(temp_path.join("spectre.yaml"))
    );
    dir.close().unwrap();
  }

  #[test]
  fn test_is_in_path() {
    let dir = Builder::new().prefix("test-file").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let test_path = temp_path.join("test.yaml");

    fs::File::create(&test_path).unwrap();
    env::set_current_dir(&temp_path).unwrap();

    let path = is_in_path(&temp_path.join("test"), "yaml");
    assert!(test_path.exists());
    assert!(path.is_some());
    dir.close().unwrap();
  }

  #[test]
  fn test_is_not_in_path() {
    let dir = Builder::new().prefix("test-file").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let test_path = temp_path.join("test.yaml");
    env::set_current_dir(&temp_path).unwrap();

    let path = is_in_path(&temp_path.join("test"), "yaml");
    assert!(!test_path.exists());
    assert!(path.is_none());
    dir.close().unwrap();
  }

  #[test]
  fn test_load() {
    use std::io::Write;

    let dir = Builder::new().prefix("test-file").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
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

    let mut config_file = fs::File::create(temp_path.join("spectre.yaml")).unwrap();
    config_file.write_all(option.as_bytes()).unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    let result = config.load(&temp_path);

    assert!(result.is_ok());
    dir.close().unwrap();
  }

  #[test]
  fn test_ext_load() {
    use std::io::Write;

    let dir = Builder::new().prefix("test-file").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
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
    let mut config_file = fs::File::create(temp_path.join("spectre.yml")).unwrap();
    config_file.write_all(option.as_bytes()).unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    let result = config.load(&temp_path);

    assert!(result.is_ok());
    dir.close().unwrap();
  }

  #[test]
  fn test_create() {
    let dir = Builder::new().prefix("test-file").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    let result = config.create(&temp_path);

    assert!(result.is_ok());
    assert!(temp_path.join("spectre.yaml").exists());
    dir.close().unwrap();
  }

  #[test]
  fn test_create_with_name() {
    let dir = Builder::new().prefix("test-file").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    config.config_name("example");
    let result = config.create(&temp_path);

    assert!(result.is_ok());
    assert!(temp_path.join("example.yaml").exists());
    dir.close().unwrap();
  }

  #[test]
  fn test_create_error() {
    let dir = Builder::new().prefix("test-file").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    fs::File::create(temp_path.join("spectre.yaml")).unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    let result = config.create(&temp_path);

    assert!(result.is_err());
    dir.close().unwrap();
  }
}
