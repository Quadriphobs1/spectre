use super::{drop_database, fresh_database, reset_database, setup_database};
use crate::util::handle_error;
use clap::ArgMatches;
use std::error::Error;

fn run_database_command(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
  match matches.subcommand() {
    ("setup", Some(args)) => setup_database(args)?,
    ("reset", Some(args)) => reset_database(args)?,
    ("fresh", Some(args)) => fresh_database(args)?,
    ("drop", Some(args)) => drop_database(args)?,
    _ => unreachable!("Unsupported command, available commands: setup, reset, fresh, drop"),
  };
  Ok(())
}

pub fn database_command(matches: &ArgMatches) {
  run_database_command(matches).unwrap_or_else(handle_error)
}
