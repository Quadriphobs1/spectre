use crate::directory::find_project_root;
use crate::ConfigOption;
use crate::Error;
use crate::Parser;
use crate::Result;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Config {
  root: PathBuf,
  config_name: String,
  config_bytes: Vec<u8>,
  extensions: Vec<String>,
  option: ConfigOption,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      root: PathBuf::new(),
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

  // Set the root folder dir
  pub fn root(&mut self, path: PathBuf) {
    self.root = path;
  }

  /// Load the config file
  pub fn load(&mut self) -> Result<()> {
    let root = find_project_root()?;
    self.root(root);

    let path_base = Path::new(&self.root).join(&self.config_name);
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
      Err(Error::FileNotFound(self.root.clone()))
    }
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
  use std::path::{Path, PathBuf};
  use std::{env, fs};

  #[test]
  fn test_config_name() {
    let mut config = Config::new();
    assert_eq!(config.config_name, String::from("spectre"));
    config.config_name("example");
    assert_eq!(config.config_name, String::from("example"));
  }

  #[test]
  fn test_root() {
    let mut config = Config::new();
    assert_eq!(config.root.to_str(), Some(""));
    config.root(PathBuf::from("file-path"));
    assert_eq!(config.root.as_path(), Path::new("file-path"));
  }

  #[test]
  fn test_load_no_root_error() {
    let dir = Builder::new().prefix("test-folder").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    let err = config.load();

    assert!(err.is_err());

    assert_eq!(
      err.unwrap_err(),
      Error::ProjectRootNotFound(PathBuf::from("test-folder"))
    );
    dir.close().unwrap();
  }

  #[test]
  fn test_file_not_exist() {
    let dir = Builder::new().prefix("test-file").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let cargo_path = temp_path.join("Cargo.toml");

    fs::File::create(&cargo_path).unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    let err = config.load();

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
    let cargo_path = temp_path.join("Cargo.toml");
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

    fs::File::create(&cargo_path).unwrap();
    let mut config_file = fs::File::create(temp_path.join("spectre.yaml")).unwrap();
    config_file.write_all(option.as_bytes()).unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    let result = config.load();

    assert!(result.is_ok());
    dir.close().unwrap();
  }

  #[test]
  fn test_ext_load() {
    use std::io::Write;

    let dir = Builder::new().prefix("test-file").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let cargo_path = temp_path.join("Cargo.toml");
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

    fs::File::create(&cargo_path).unwrap();
    let mut config_file = fs::File::create(temp_path.join("spectre.yml")).unwrap();
    config_file.write_all(option.as_bytes()).unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    let result = config.load();

    assert!(result.is_ok());
    dir.close().unwrap();
  }
}
