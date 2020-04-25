use crate::result::CliResult;
use std::fmt;
use std::str::FromStr;

// Supported database type enum
#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
pub enum DatabaseOption {
  #[cfg(feature = "postgres")]
  #[serde(rename(serialize = "postgres", deserialize = "postgres"))]
  Postgres,
  #[cfg(feature = "mysql")]
  #[serde(rename(serialize = "mysql", deserialize = "mysql"))]
  MySQL,
}

// Implement the trait
impl FromStr for DatabaseOption {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      #[cfg(feature = "postgres")]
      "postgres" => Ok(DatabaseOption::Postgres),
      #[cfg(feature = "mysql")]
      "mysql" => Ok(DatabaseOption::MySQL),
      _ => Err("supported databases: postgres, mysql"),
    }
  }
}

impl fmt::Display for DatabaseOption {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match *self {
      #[cfg(feature = "postgres")]
      DatabaseOption::Postgres => f.write_str("postgres"),
      #[cfg(feature = "mysql")]
      DatabaseOption::MySQL => f.write_str("mysql"),
    }
  }
}

impl Default for DatabaseOption {
  fn default() -> Self {
    DatabaseOption::Postgres
  }
}

impl DatabaseOption {
  fn default() -> Self {
    DatabaseOption::Postgres
  }

  pub fn verify_url(&self, db_url: &str) -> CliResult<()> {
    match self {
      #[cfg(feature = "postgres")]
      DatabaseOption::Postgres => {
        if db_url.starts_with("postgres://") || db_url.starts_with("postgresql://") {
          Ok(())
        } else {
          panic!("Invalid database url `{}` provided for `postgres`", db_url);
        }
      }
      #[cfg(feature = "mysql")]
      DatabaseOption::MySQL => {
        if db_url.starts_with("mysql://") {
          Ok(())
        } else {
          panic!("Invalid database url `{}` provided for `mysql`", db_url);
        }
      }
    }
  }

  pub fn variants() -> [&'static str; 2] {
    ["postgres", "mysql"]
  }
}

#[derive(Default, Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DatabaseConfig {
  pub logging: bool,
  #[serde(default = "DatabaseOption::default")]
  pub provider: DatabaseOption,
}

impl DatabaseConfig {
  pub fn from(db: DatabaseOption) -> Self {
    DatabaseConfig {
      logging: false,
      provider: db,
    }
  }
}
