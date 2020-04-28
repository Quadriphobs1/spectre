#[macro_use]
extern crate clap;
extern crate config;
extern crate dotenv;
extern crate toml;

mod cli;
mod connection;
mod database;
mod directory;
mod driver;
mod errors;
mod init;
mod migrate;
mod seed;
mod util;

use self::database::database_command;
use self::errors::Error;
use self::init::init_command;
use self::migrate::migrate_command;
use self::seed::seed_command;
use clap::{crate_name, ArgMatches, Shell};
use std::io::stdout;

type Result<T> = std::result::Result<T, Error>;

pub fn main() {
  use self::dotenv::dotenv;
  dotenv().ok();

  let matches = cli::build_cli().get_matches();

  match matches.subcommand() {
    ("init", Some(matches)) => init_command(matches),
    ("seed", Some(matches)) => seed_command(matches),
    ("migrate", Some(matches)) => migrate_command(matches),
    ("database", Some(matches)) => database_command(matches),
    ("completions", Some(matches)) => completions_command(matches),
    _ => unreachable!("Unsupported command"),
  }
}

fn completions_command(matches: &ArgMatches) {
  let shell = value_t!(matches, "shell", Shell).unwrap_or_else(|e| e.exit());
  cli::build_cli().gen_completions_to(crate_name!(), shell, &mut stdout());
}
