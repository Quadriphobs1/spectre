use crate::result::{CliError, CliResult};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_project_root() -> CliResult<PathBuf> {
  let cur_dir = env::current_dir()?;
  search_for_directory_containing_file(&cur_dir, "Cargo.toml")
}

pub fn search_for_directory_containing_file(path: &Path, file: &str) -> CliResult<PathBuf> {
  let toml_path = path.join(file);
  if toml_path.is_file() {
    Ok(path.to_owned())
  } else {
    path
      .parent()
      .map(|p| search_for_directory_containing_file(p, file))
      .unwrap_or_else(|| Err(CliError::ProjectRootNotFound(path.into())))
      .map_err(|_| CliError::ProjectRootNotFound(path.into()))
  }
}

pub fn verify_or_create_directory(base: &PathBuf, dir: String) -> Result<(), Box<dyn Error>> {
  let dir_path = Path::new(&base).join(dir);
  if !dir_path.is_dir() {
    fs::create_dir_all(dir_path)?;
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  extern crate tempfile;
  use self::tempfile::Builder;
  use crate::result::CliError;

  use super::{
    find_project_root, search_for_directory_containing_file, verify_or_create_directory,
  };
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
      Err(CliError::ProjectRootNotFound(temp_path.clone())),
      search_for_directory_containing_file(&temp_path, "Cargo.toml")
    );
    dir.close().unwrap();
  }

  #[test]
  fn should_create_dir() {
    let dir = Builder::new().prefix("test4").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();

    assert!(verify_or_create_directory(&temp_path, String::from("sample")).is_ok());

    assert!(temp_path.join("sample").is_dir());
    dir.close().unwrap();
  }

  #[test]
  fn should_create_nested_dir() {
    let dir = Builder::new().prefix("test7").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();

    assert!(verify_or_create_directory(&temp_path, String::from("sample/nested")).is_ok());

    assert!(temp_path.join("sample").is_dir());
    assert!(temp_path.join("sample/nested").is_dir());
    dir.close().unwrap();
  }

  #[test]
  fn should_not_create_dir() {
    let dir = Builder::new().prefix("test5").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    fs::create_dir(&temp_path.join("sample")).unwrap();

    assert!(temp_path.join("sample").is_dir());
    assert!(verify_or_create_directory(&temp_path, String::from("sample")).is_ok());

    dir.close().unwrap();
  }

  #[test]
  fn should_get_project_root() {
    let dir = Builder::new().prefix("test6").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let toml_path = temp_path.join("Cargo.toml");

    fs::File::create(&toml_path).unwrap();
    env::set_current_dir(&temp_path).unwrap();
    assert_eq!(Ok(temp_path.clone()), find_project_root());
    dir.close().unwrap();
  }
}
