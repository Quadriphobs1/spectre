use super::database_url;
use crate::Result;
use clap::ArgMatches;

pub fn setup_database(_: &ArgMatches) -> Result<()> {
  let _database_url = database_url();
  // ensure there is a connection to the database
  // Create default migration table
  Ok(())
}
