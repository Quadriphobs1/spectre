use std::convert::From;
use std::error::Error as StdError;
use std::path::PathBuf;
use std::{fmt, io};

use self::Error::*;

#[derive(Debug)]
pub enum Error {
  ProjectRootNotFound(PathBuf),
  FileNotFound(PathBuf),
  IoError(io::Error),
  ConfigFileError(serde_yaml::Error),
  #[doc(hidden)]
  __Nonexhaustive,
}

impl Error {
  pub fn not_found(&self) -> bool {
    if let Error::IoError(ref io_error) = *self {
      return io_error.kind() == io::ErrorKind::NotFound;
    }
    false
  }
}

impl From<io::Error> for Error {
  fn from(e: io::Error) -> Self {
    IoError(e)
  }
}

impl From<serde_yaml::Error> for Error {
  fn from(e: serde_yaml::Error) -> Self {
    ConfigFileError(e)
  }
}

impl StdError for Error {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self {
      IoError(err) => Some(err),
      ConfigFileError(err) => Some(err),
      _ => None,
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ProjectRootNotFound(ref p) => write!(
        f,
        "Unable to find Cargo.toml in {:?} or any parent directories.",
        p
      ),
      FileNotFound(ref p) => write!(
        f,
        "Unable to find file or folder in {:?} parent directories.",
        p
      ),
      IoError(ref error) => f.write_str(
        &error
          .source()
          .map(ToString::to_string)
          .unwrap_or_else(|| error.to_string()),
      ),
      ConfigFileError(ref error) => f.write_str(
        &error
          .source()
          .map(ToString::to_string)
          .unwrap_or_else(|| error.to_string()),
      ),
      _ => unreachable!(),
    }
  }
}

impl PartialEq for Error {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&ProjectRootNotFound(_), &ProjectRootNotFound(_)) => true,
      (&FileNotFound(_), &FileNotFound(_)) => true,
      _ => false,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::error::Error as StdError;
  use std::path::PathBuf;
  use unindent::unindent;

  #[test]
  fn test_io_error_source() {
    let err = Error::IoError(std::io::ErrorKind::PermissionDenied.into());
    let io_err = err
      .source()
      .unwrap()
      .downcast_ref::<std::io::Error>()
      .unwrap();
    assert_eq!(std::io::ErrorKind::PermissionDenied, io_err.kind());
  }

  #[test]
  fn test_error_not_found_true() {
    let err = Error::IoError(std::io::ErrorKind::NotFound.into());
    assert!(err.not_found());
  }

  #[test]
  fn test_error_not_found_false() {
    let err = Error::IoError(std::io::ErrorKind::PermissionDenied.into());
    assert!(!err.not_found());
  }

  #[test]
  fn test_error_not_found_false_default() {
    let err = Error::ProjectRootNotFound(PathBuf::from("sample"));
    assert!(!err.not_found());
  }

  #[test]
  fn test_io_error_display() {
    let err = Error::IoError(std::io::ErrorKind::PermissionDenied.into());
    let io_err: std::io::Error = std::io::ErrorKind::PermissionDenied.into();

    let err_desc = format!("{}", err);
    let io_err_desc = format!("{}", io_err);
    assert_eq!(io_err_desc, err_desc);
  }

  #[test]
  fn test_project_root_not_found_source() {
    let err = Error::ProjectRootNotFound(PathBuf::from("sample"));
    assert!(err.source().is_none());
  }

  #[test]
  fn test_project_root_not_found_error_display() {
    let err = Error::ProjectRootNotFound(PathBuf::from("sample"));
    let err_desc = format!("{}", err);
    assert_eq!(
      format!(
        "Unable to find Cargo.toml in {:?} or any parent directories.",
        PathBuf::from("sample")
      ),
      err_desc
    );
  }

  #[test]
  fn test_file_not_found_error_display() {
    let err = Error::FileNotFound(PathBuf::from("sample.toml"));
    let err_desc = format!("{}", err);
    assert_eq!(
      format!(
        "Unable to find file or folder in {:?} parent directories.",
        PathBuf::from("sample.toml")
      ),
      err_desc
    );
  }

  #[test]
  fn test_equal() {
    let err = Error::FileNotFound(PathBuf::from("sample.toml"));
    assert_eq!(err, Error::FileNotFound(PathBuf::from("sample.toml")));
  }

  #[test]
  fn test_not_equal() {
    let err = Error::IoError(std::io::ErrorKind::PermissionDenied.into());
    assert_ne!(
      err,
      Error::IoError(std::io::ErrorKind::PermissionDenied.into())
    );
  }

  #[test]
  fn test_config_file_error() {
    #[derive(Deserialize, Debug)]
    struct Basic {
      v: bool,
      w: bool,
    }
    let yaml = unindent(
      "
        ---
        v: true",
    );

    let err = Error::ConfigFileError(serde_yaml::from_str::<Basic>(&yaml).unwrap_err());
    let expected_desc = "missing field `w` at line 2 column 2".to_string();
    let err_desc = format!("{}", err);
    assert_eq!(expected_desc, err_desc);
  }
}
