use crate::config::Config;
use crate::driver::{Driver, DriverFactory};
use crate::result::ConnectionResult;

pub struct Connection<T: Driver> {
  _name: String,
  _driver: T,
  // logger
  _is_connected: bool,
}

impl<T: Driver> Connection<T> {
  pub fn establish(config: &Config, database_url: &str) -> Self {
    let _driver: T = DriverFactory::create(config.database.provider, database_url);
    unimplemented!()
  }

  pub fn _close(&self) -> ConnectionResult<()> {
    // self.driver.disconnect()?;
    Ok(())
  }
}
