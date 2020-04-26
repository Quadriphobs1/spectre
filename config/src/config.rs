use crate::directory::find_project_root;
use crate::ConfigOption;
use crate::Error;
use crate::Parser;
use crate::Result;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
  root: PathBuf,
  config_name: String,
  config_bytes: Vec<u8>,
  option: ConfigOption,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      root: PathBuf::new(),
      config_name: String::from("spectre.toml"),
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
    let path = Path::new(&self.root).join(&self.config_name);

    if path.exists() {
      let mut bytes = Vec::new();
      fs::File::open(&path)?.read_to_end(&mut bytes)?;
      self.config_bytes = bytes;
      let option = Parser::into(&self.config_bytes)?;
      self.option = option;
      Ok(())
    } else {
      Err(Error::FileNotFound(path))
    }
  }
}

#[cfg(test)]
mod tests {
  extern crate tempfile;
  use self::tempfile::Builder;
  use super::Config;
  use crate::Error;
  use std::path::{Path, PathBuf};
  use std::{env, fs};

  #[test]
  fn test_config_name() {
    let mut config = Config::new();
    assert_eq!(config.config_name, String::from("spectre.toml"));
    config.config_name("example.toml");
    assert_eq!(config.config_name, String::from("example.toml"));
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
    let toml_path = temp_path.join("Cargo.toml");

    fs::File::create(&toml_path).unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let mut config = Config::new();
    let err = config.load();

    assert!(err.is_err());

    assert_eq!(
      err.unwrap_err(),
      Error::FileNotFound(temp_path.join("spectre.toml"))
    );
    dir.close().unwrap();
  }
}
