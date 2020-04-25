use super::database_url;

use crate::config::Config;
use crate::connection::Connection;
use crate::driver::PgDriver;
use crate::result::{CliError, CliResult};
use crate::util::handle_error;
use clap::ArgMatches;

pub fn setup_database(_: &ArgMatches) -> CliResult<()> {
  let database_url = database_url();
  let config = Config::load().unwrap_or_else(|_| handle_error(CliError::ConfigLoadError));
  config.database.provider.verify_url(&database_url)?;
  let _conn: Connection<PgDriver> = Connection::establish(&config, &database_url);
  // ensure there is a connection to the database
  // Create default migration table
  Ok(())
}
