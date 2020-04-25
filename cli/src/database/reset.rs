use super::database_url;
use clap::ArgMatches;
use std::error::Error;

pub fn reset_database(_args: &ArgMatches) -> Result<(), Box<dyn Error>> {
  let _db_url = database_url();
  // load config
  unimplemented!();
}
