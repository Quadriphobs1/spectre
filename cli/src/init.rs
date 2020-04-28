use crate::directory::find_project_root;
use crate::util::handle_error;
use clap::ArgMatches;
use config::Config;

pub fn init_command(matches: &ArgMatches) {
  let base_path = match find_project_root() {
    Ok(path) => path,
    Err(error) => {
      println!("{}", error);
      ::std::process::exit(1);
    }
  };

  let name = value_t!(matches, "name", String).unwrap();
  let docker = value_t!(matches, "docker", bool).unwrap_or(false);

  let mut config = Config::new();
  if !name.is_empty() {
    config.config_name(&name);
  }
  config.create(&base_path).unwrap_or_else(handle_error);

  // TODO: Handle docker compose file creation
  if docker {
    println!("Yet to implement this, Hold on");
    // create_docker_compose_file(&base_path, database).unwrap_or_else(handle_error);
  }
  println!(
    "Project created inside {:?} directory.",
    base_path.to_str().unwrap()
  );
}

// pub fn create_docker_compose_file(
//   base: &PathBuf,
//   database: DatabaseOption,
// ) -> Result<(), Box<dyn Error>> {
//   // TODO: docker-compose should ask for database to support and create for each of them
//   use std::io::Write;

//   let path = base.join("docker-compose.yml");
//   if !path.exists() {
//     let template = get_docker_compose_template(&database);
//     let mut file = fs::File::create(path)?;
//     file.write_all(&template)?;
//   }

//   Ok(())
// }

// fn get_docker_compose_template(database: &DatabaseOption) -> &[u8] {
//   match database {
//     #[cfg(feature = "mysql")]
//     DatabaseOption::MySQL => {
//       b"
// version: '3'
// services:
//   mysql:
//     image: mysql
//     ports:
//       - 3306:3306
//     environment:
//       MYSQL_ROOT_PASSWORD: admin
//       MYSQL_USER: test
//       MYSQL_PASSWORD: test
//       MYSQL_DATABASE: test
//     "
//     }
//     #[cfg(feature = "postgres")]
//     DatabaseOption::Postgres => {
//       b"
// version: '3'
// services:
//   postgres:
//     image: postgres
//     ports:
//       - 5432:5432
//     environment:
//       POSTGRES_USER: test
//       POSTGRES_PASSWORD: test
//       POSTGRES_DB: test
//     "
//     }
//   }
// }

// #[cfg(test)]
// mod tests {
//   extern crate tempfile;

//   use self::tempfile::Builder;
//   use super::{create_docker_compose_file};
//   use crate::database::DatabaseOption;
//   use std::env;

//   #[test]
//   fn should_create_docker_compose_file() {
//     let dir = Builder::new().prefix("test3").tempdir().unwrap();
//     let temp_path = dir.path().canonicalize().unwrap();
//     env::set_current_dir(&temp_path).unwrap();
//     let db = DatabaseOption::Postgres;
//     assert!(create_docker_compose_file(&temp_path, db).is_ok());
//     assert!(temp_path.join("docker-compose.yml").exists());
//   }
// }
