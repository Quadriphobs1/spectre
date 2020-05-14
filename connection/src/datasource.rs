use crate::Result;
use std::fmt;
use std::str::FromStr;

use self::Provider::*;

#[derive(Deserialize, Debug, Hash, Clone, Serialize)]
pub enum Provider {
  #[cfg(feature = "postgres")]
  #[serde(rename(serialize = "postgres", deserialize = "postgres"))]
  Postgres,
  #[cfg(feature = "mysql")]
  #[serde(rename(serialize = "mysql", deserialize = "mysql"))]
  MySQL,
  #[cfg(feature = "sqlite")]
  #[serde(rename(serialize = "sqlite", deserialize = "sqlite"))]
  SQLite,
}

impl FromStr for Provider {
  type Err = &'static str;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      #[cfg(feature = "postgres")]
      "postgres" => Ok(Postgres),
      #[cfg(feature = "mysql")]
      "mysql" => Ok(MySQL),
      #[cfg(feature = "sqlite")]
      "sqlite" => Ok(SQLite),
      _ => Err("supported databases: postgres, mysql, sqlite"),
    }
  }
}

impl fmt::Display for Provider {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
    match *self {
      #[cfg(feature = "postgres")]
      Postgres => f.write_str("postgres"),
      #[cfg(feature = "mysql")]
      MySQL => f.write_str("mysql"),
      #[cfg(feature = "sqlite")]
      SQLite => f.write_str("sqlite"),
    }
  }
}

impl Default for Provider {
  fn default() -> Self {
    Postgres
  }
}

impl Provider {
  pub fn verify_url(self, url: &str) -> Result<()> {
    match self {
      #[cfg(feature = "postgres")]
      Postgres => {
        if url.starts_with("postgres://") || url.starts_with("postgresql://") {
          Ok(())
        } else {
          panic!("Invalid database url `{}` provided for `postgres`", url);
        }
      }
      #[cfg(feature = "mysql")]
      MySQL => {
        if url.starts_with("mysql://") {
          Ok(())
        } else {
          panic!("Invalid database url `{}` provided for `mysql`", url);
        }
      }
      #[cfg(feature = "sqlite")]
      SQLite => {
        if url.starts_with("sqlite://") {
          Ok(())
        } else {
          panic!("Invalid database url `{}` provided for `sqlite`", url);
        }
      }
    }
  }

  /// A list of possible variants in `&'static str` form
  pub fn variants() -> [&'static str; 3] {
    ["postgres", "mysql", "sqlite"]
  }
}

#[derive(Deserialize, Debug, Hash, Clone, Serialize)]
pub struct Datasource {
  #[serde(default)]
  pub provider: Provider,
  #[serde(default)]
  pub url: String,
}

impl Default for Datasource {
  fn default() -> Self {
    Datasource {
      provider: Provider::default(),
      url: "postgresql://postgres:admin@localhost:5432/postgres?schema=public".to_string(),
    }
  }
}
