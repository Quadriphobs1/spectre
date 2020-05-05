use std::path::{Path, PathBuf};
use std::{env, error, fmt, io};

use self::Error::*;

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

#[derive(Debug)]
pub enum Error {
  ProjectRootNotFound(PathBuf),
  IoError(io::Error),
  #[doc(hidden)]
  __Nonexhaustive,
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    IoError(e)
  }
}

impl error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ProjectRootNotFound(ref p) => write!(
        f,
        "Unable to find Cargo.toml in {:?} or any parent directories.",
        p
      ),
      _ => unreachable!(),
    }
  }
}

impl PartialEq for Error {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&ProjectRootNotFound(_), &ProjectRootNotFound(_)) => true,
      _ => false,
    }
  }
}

type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
  extern crate tempfile;
  use crate::Error;

  use super::{find_project_root, search_for_directory_containing_file};
  use std::fs;
  use std::{env, path::PathBuf};

  #[test]
  fn find_cargo_toml() {
    let dir = tempfile::tempdir().unwrap();
    let temp_path = dir.path();
    let toml_path = temp_path.join("Cargo.toml");

    fs::File::create(&toml_path).unwrap();

    assert_eq!(
      Ok(temp_path.into()),
      search_for_directory_containing_file(&temp_path, "Cargo.toml")
    );
    dir.close().unwrap();
  }

  #[test]
  fn cargo_toml_in_root() {
    let dir = tempfile::tempdir().unwrap();
    let temp_path = dir.path();
    let toml_path = temp_path.join("Cargo.toml");

    fs::File::create(&toml_path).unwrap();
    let nested_path = temp_path.join("nested");
    fs::create_dir(&nested_path).unwrap();
    let deeply_nested = &nested_path.join("deeply_nested");
    fs::create_dir(deeply_nested).unwrap();
    assert_eq!(
      Ok(temp_path.into()),
      search_for_directory_containing_file(&temp_path, "Cargo.toml")
    );
    dir.close().unwrap();
  }

  #[test]
  fn not_found_if_no_cargo_toml() {
    let dir = tempfile::tempdir().unwrap();
    let temp_path = dir.path();
    assert_eq!(
      Err(Error::ProjectRootNotFound(temp_path.into())),
      search_for_directory_containing_file(&temp_path, "Cargo.toml")
    );
    dir.close().unwrap();
  }

  #[test]
  fn should_get_project_root() {
    let dir = tempfile::tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let toml_path = temp_path.join("Cargo.toml");

    fs::File::create(&toml_path).unwrap();
    let cur_dir = env::current_dir().unwrap();
    env::set_current_dir(&temp_path).unwrap();
    assert_eq!(Ok(temp_path), find_project_root());
    env::set_current_dir(&cur_dir).unwrap();
    dir.close().unwrap();
  }

  #[test]
  fn test_equal() {
    let err = Error::ProjectRootNotFound(PathBuf::from("sample"));
    assert_eq!(err, Error::ProjectRootNotFound(PathBuf::from("sample")));
  }

  #[test]
  fn test_not_equal() {
    let err = Error::IoError(std::io::ErrorKind::PermissionDenied.into());
    assert_ne!(
      err,
      Error::IoError(std::io::ErrorKind::PermissionDenied.into())
    );
  }
}
