use crate::config::Config;
use crate::driver::{Driver, DriverFactory};
use crate::result::ConnectionResult;

pub struct Connection<T: Driver> {
  name: String,
  driver: T,
  // logger
  is_connected: bool,
}

impl<T: Driver> Connection<T> {
  pub fn establish(config: &Config, database_url: &str) -> Self {
    let driver: T = DriverFactory::create(&config.database.provider, database_url);
    unimplemented!()
  }

  pub fn close(&self) -> ConnectionResult<()> {
    self.driver.disconnect()?;
    Ok(())
  }
}
