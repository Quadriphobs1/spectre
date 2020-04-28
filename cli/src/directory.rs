use crate::{Error, Result};
use std::env;
use std::path::{Path, PathBuf};

pub fn find_project_root() -> Result<PathBuf> {
  let cur_dir = env::current_dir()?;
  search_for_directory_containing_file(&cur_dir, "Cargo.toml")
}

pub fn search_for_directory_containing_file(path: &Path, file: &str) -> Result<PathBuf> {
  let toml_path = path.join(file);
  if toml_path.is_file() {
    Ok(path.to_owned())
  } else {
    path
      .parent()
      .map(|p| search_for_directory_containing_file(p, file))
      .unwrap_or_else(|| Err(Error::ProjectRootNotFound(path.into())))
      .map_err(|_| Error::ProjectRootNotFound(path.into()))
  }
}

#[cfg(test)]
mod tests {
  extern crate tempfile;
  use self::tempfile::Builder;
  use crate::Error;

  use super::{find_project_root, search_for_directory_containing_file};
  use std::env;
  use std::fs;

  #[test]
  fn find_cargo_toml() {
    let dir = Builder::new().prefix("test1").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let toml_path = temp_path.join("Cargo.toml");

    fs::File::create(&toml_path).unwrap();

    assert_eq!(
      Ok(temp_path.clone()),
      search_for_directory_containing_file(&temp_path, "Cargo.toml")
    );
    dir.close().unwrap();
  }

  #[test]
  fn cargo_toml_in_root() {
    let dir = Builder::new().prefix("test2").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let toml_path = temp_path.join("Cargo.toml");

    fs::File::create(&toml_path).unwrap();
    let nested_path = temp_path.join("nested");
    fs::create_dir(&nested_path).unwrap();
    let deeply_nested = &nested_path.join("deeply_nested");
    fs::create_dir(deeply_nested).unwrap();
    env::set_current_dir(&deeply_nested).unwrap();
    assert_eq!(
      Ok(temp_path.clone()),
      search_for_directory_containing_file(&temp_path, "Cargo.toml")
    );
    dir.close().unwrap();
  }

  #[test]
  fn not_found_if_no_cargo_toml() {
    let dir = Builder::new().prefix("test3").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();

    assert_eq!(
      Err(Error::ProjectRootNotFound(temp_path.clone())),
      search_for_directory_containing_file(&temp_path, "Cargo.toml")
    );
    dir.close().unwrap();
  }

  #[test]
  fn should_get_project_root() {
    let dir = Builder::new().prefix("test6").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let toml_path = temp_path.join("Cargo.toml");

    fs::File::create(&toml_path).unwrap();
    env::set_current_dir(&temp_path).unwrap();
    assert_eq!(Ok(temp_path), find_project_root());
    dir.close().unwrap();
  }
}
