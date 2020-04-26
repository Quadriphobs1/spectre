use crate::database::DatabaseOption;

pub trait Driver: Sized + Send {
  fn establish(database_url: &str) -> Result<Self, Box<dyn std::error::Error>>;
  fn connect(&self) -> Result<(), Box<dyn std::error::Error>>;
  fn disconnect(&self) -> Result<(), Box<dyn std::error::Error>>;
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
  fn establish(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let conn = PgDriver {
      database: String::from(database_url),
    };
    Ok(conn)
  }

  fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }

  fn disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
  }
}
