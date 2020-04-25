use std::convert::From;
use std::error::Error;
use std::ffi::NulError;
use std::path::PathBuf;

use std::{fmt, io};

use self::CliError::*;

pub type ConnectionResult<T> = Result<T, ConnectionError>;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum ConnectionError {
  /// The connection URL contained a `NUL` byte.
  InvalidString(NulError),
  BadConnection(String),
  /// The connection URL could not be parsed.
  InvalidConnectionUrl(String),
}

impl From<NulError> for ConnectionError {
  fn from(e: NulError) -> Self {
    ConnectionError::InvalidString(e)
  }
}

impl fmt::Display for ConnectionError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ConnectionError::InvalidString(ref nul_err) => nul_err.fmt(f),
      ConnectionError::BadConnection(ref s) => write!(f, "{}", s),
      ConnectionError::InvalidConnectionUrl(ref s) => write!(f, "{}", s),
    }
  }
}

impl Error for ConnectionError {
  fn cause(&self) -> Option<&dyn Error> {
    match *self {
      ConnectionError::InvalidString(ref e) => Some(e),
      _ => None,
    }
  }
}

#[derive(Debug)]
pub enum CliError {
  ProjectRootNotFound(PathBuf),
  ConfigLoadError,
  IoError(io::Error),
  MigrationsDirectoryNotFound(PathBuf),
  SeedsDirectoryNotFound(PathBuf),
  DatabaseUrlMissing,
}

impl From<io::Error> for CliError {
  fn from(e: io::Error) -> Self {
    IoError(e)
  }
}

impl Error for CliError {}

impl fmt::Display for CliError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      ProjectRootNotFound(ref p) => write!(
        f,
        "Unable to find Cargo.toml in {:?} or any parent directories.",
        p
      ),
      MigrationsDirectoryNotFound(ref p) => {
        write!(f, "Unable to find migrations directory in {:?}.", p)
      }
      SeedsDirectoryNotFound(ref p) => write!(f, "Unable to find seeds directory in {:?}.", p),
      DatabaseUrlMissing => f.write_str("The the DATABASE_URL environment variable must be set."),
      ConfigLoadError => f.write_str("Unable to find the configuration file, run `spectre init` to setup the configuration file."),
      IoError(ref error) => f.write_str(
        &error
          .source()
          .map(ToString::to_string)
          .unwrap_or_else(|| error.to_string()),
      ),
    }
  }
}

impl PartialEq for CliError {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (&ProjectRootNotFound(_), &ProjectRootNotFound(_)) => true,
      (&MigrationsDirectoryNotFound(_), &MigrationsDirectoryNotFound(_)) => true,
      (&SeedsDirectoryNotFound(_), &SeedsDirectoryNotFound(_)) => true,
      _ => false,
    }
  }
}
