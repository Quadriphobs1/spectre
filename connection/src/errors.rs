use std::convert::From;
use std::ffi::NulError;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
  /// The connection URL contained a `NUL` byte.
  InvalidString(NulError),
  BadConnection(String),
  /// The connection URL could not be parsed.
  InvalidConnectionUrl(String),
  #[doc(hidden)]
  __Nonexhaustive,
}

impl From<NulError> for Error {
  fn from(e: NulError) -> Self {
    Error::InvalidString(e)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::InvalidString(ref nul_err) => nul_err.fmt(f),
      Error::BadConnection(ref s) => write!(f, "{}", s),
      Error::InvalidConnectionUrl(ref s) => write!(f, "{}", s),
      _ => unreachable!(),
    }
  }
}

impl error::Error for Error {
  fn cause(&self) -> Option<&dyn error::Error> {
    match *self {
      Error::InvalidString(ref e) => Some(e),
      _ => None,
    }
  }

  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Error::InvalidString(err) => Some(err),
      _ => None,
    }
  }
}

#[cfg(test)]
mod test {
  use std::error::Error as StdError;

  use super::*;

  #[test]
  fn test_bad_connection_error() {
    let err = Error::BadConnection(String::from("Error establishing connection"));
    assert!(err.source().is_none());
  }
  #[test]
  fn test_invalid_connection_url() {
    let err = Error::InvalidConnectionUrl(String::from("Invalid connection URL provided"));
    assert!(err.source().is_none());
  }
}
