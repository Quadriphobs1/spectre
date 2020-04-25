use super::init_files::{create_config_file, create_docker_compose_file};
use crate::database::DatabaseOption;
use crate::directory::{find_project_root, verify_or_create_directory};
use crate::util::handle_error;
use clap::ArgMatches;
use std::path::PathBuf;

pub fn init_command(matches: &ArgMatches) {
  let base_path = match find_project_root() {
    Ok(path) => path,
    Err(error) => {
      println!("{}", error);
      ::std::process::exit(1);
    }
  };

  let database = value_t!(matches, "database", DatabaseOption).unwrap_or(DatabaseOption::Postgres);

  let migrations_dir =
    value_t!(matches, "migrations", String).unwrap_or_else(|_| String::from("src/migrations"));
  let seeds_dir = value_t!(matches, "seeds", String).unwrap_or_else(|_| String::from("src/seeds"));
  // Create the seeds and migrations directory if it doesn't yet exist
  verify_or_create_directory(&base_path, migrations_dir.clone()).unwrap_or_else(handle_error);
  verify_or_create_directory(&base_path, seeds_dir.clone()).unwrap_or_else(handle_error);

  let docker = value_t!(matches, "docker", bool).unwrap_or(false);

  if docker {
    create_docker_compose_file(&base_path, database).unwrap_or_else(handle_error);
  }

  // Log output for user on the status of the operation
  create_config_file(
    &base_path,
    database,
    PathBuf::from(migrations_dir),
    PathBuf::from(seeds_dir),
  )
  .unwrap_or_else(handle_error);
  println!(
    "Project created inside {:?} directory.",
    base_path.to_str().unwrap()
  );
}
