mod mysql;
mod postgres;
mod sqlite;

use self::mysql::MySQLDriver;
use self::postgres::PotsgresDriver;
use self::sqlite::SQLiteDriver;
use crate::datasource::Provider;

use crate::Result;

pub trait Driver: std::fmt::Debug {
  // const whatervre: whatevr

  fn connect(&self) -> Result<()>;

  fn disconnect(&self) -> Result<()>;

  fn post_connect(&self) -> Result<()>;

  fn establish(database_url: &str) -> Result<Self>
  where
    Self: Sized;
}

pub fn create_driver(source: &Provider) -> Box<dyn Driver + 'static> {
  match source {
    Provider::Postgres => Box::new(PotsgresDriver::new()),
    Provider::MySQL => Box::new(MySQLDriver::new()),
    Provider::SQLite => Box::new(SQLiteDriver::new()),
  }
}
