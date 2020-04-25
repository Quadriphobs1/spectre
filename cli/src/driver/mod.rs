use crate::database::DatabaseOption;
use crate::result::ConnectionResult;

pub trait Driver: Sized + Send {
  fn establish(database_url: &str) -> ConnectionResult<Self>;
  fn connect(&self) -> ConnectionResult<()>;
  fn disconnect(&self) -> ConnectionResult<()>;
}

pub struct DriverFactory;

impl DriverFactory {
  pub fn create<T: Driver>(database: DatabaseOption, _database_url: &str) -> T {
    match database {
      #[cfg(feature = "postgres")]
      DatabaseOption::Postgres => unimplemented!(),
      #[cfg(feature = "mysql")]
      DatabaseOption::MySQL => unimplemented!(),
    }
  }
}

pub struct PgDriver {
  database: String,
}

unsafe impl Send for PgDriver {}

impl Driver for PgDriver {
  fn establish(database_url: &str) -> ConnectionResult<Self> {
    let conn = PgDriver {
      database: String::from(database_url),
    };
    Ok(conn)
  }

  fn connect(&self) -> ConnectionResult<()> {
    Ok(())
  }

  fn disconnect(&self) -> ConnectionResult<()> {
    Ok(())
  }
}
