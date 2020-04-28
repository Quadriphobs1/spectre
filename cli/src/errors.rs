use std::convert::From;
use std::error::Error as StdError;
use std::path::PathBuf;

use std::{fmt, io};

use self::Error::*;

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

impl StdError for Error {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self {
      IoError(err) => Some(err),
      _ => None,
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      ProjectRootNotFound(ref p) => write!(
        f,
        "Unable to find Cargo.toml in {:?} or any parent directories.",
        p
      ),
      IoError(ref error) => f.write_str(
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
      _ => false,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

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
  fn test_io_error_display() {
    let err = Error::IoError(std::io::ErrorKind::PermissionDenied.into());
    let io_err: std::io::Error = std::io::ErrorKind::PermissionDenied.into();

    let err_desc = format!("{}", err);
    let io_err_desc = format!("{}", io_err);
    assert_eq!(io_err_desc, err_desc);
  }

  #[test]
  fn test_root_not_found_display() {
    let err = Error::ProjectRootNotFound(PathBuf::from("testing"));
    let err_desc = format!("{}", err);
    assert_eq!(
      format!(
        "Unable to find Cargo.toml in {:?} or any parent directories.",
        PathBuf::from("testing")
      ),
      err_desc
    );
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
