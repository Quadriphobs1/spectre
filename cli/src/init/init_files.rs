use crate::config::{Config, MigrationsDirectory, SeedsDirectory};
use crate::database::{DatabaseConfig, DatabaseOption};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn create_config_file(
  base: &PathBuf,
  database: DatabaseOption,
  migrations: PathBuf,
  seeds: PathBuf,
) -> Result<(), Box<dyn Error>> {
  use std::io::Write;
  let config = Config {
    database: DatabaseConfig::from(database),
    migrations: Some(MigrationsDirectory { dir: migrations }),
    seeds: Some(SeedsDirectory { dir: seeds }),
  };
  let toml = toml::to_string(&config).unwrap();
  let path = base.join("spectre.toml");
  if !path.exists() {
    let mut file = fs::File::create(path)?;
    file.write_all(toml.as_bytes())?;
  }

  Ok(())
}

pub fn create_docker_compose_file(
  base: &PathBuf,
  database: DatabaseOption,
) -> Result<(), Box<dyn Error>> {
  use std::io::Write;

  let path = base.join("docker-compose.yml");
  if !path.exists() {
    let template = get_docker_compose_template(&database);
    let mut file = fs::File::create(path)?;
    file.write_all(&template)?;
  }

  Ok(())
}

fn get_docker_compose_template(database: &DatabaseOption) -> &[u8] {
  match database {
    #[cfg(feature = "mysql")]
    DatabaseOption::MySQL => {
      b"
version: '3'
services:
  mysql:
    image: mysql
    ports:
      - 3306:3306
    environment:
      MYSQL_ROOT_PASSWORD: admin
      MYSQL_USER: test
      MYSQL_PASSWORD: test
      MYSQL_DATABASE: test
    "
    }
    #[cfg(feature = "postgres")]
    DatabaseOption::Postgres => {
      b"
version: '3'
services:
  postgres:
    image: postgres
    ports:
      - 5432:5432
    environment:
      POSTGRES_USER: test
      POSTGRES_PASSWORD: test
      POSTGRES_DB: test
    "
    }
  }
}

#[cfg(test)]
mod tests {
  extern crate tempfile;

  use self::tempfile::Builder;
  use super::{create_config_file, create_docker_compose_file};
  use crate::database::DatabaseOption;
  use std::env;
  use std::path::PathBuf;

  #[test]
  fn should_create_docker_compose_file() {
    let dir = Builder::new().prefix("test3").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let db = DatabaseOption::Postgres;
    assert!(create_docker_compose_file(&temp_path, db).is_ok());
    assert!(temp_path.join("docker-compose.yml").exists());
  }

  #[test]
  fn should_create_config_file() {
    let dir = Builder::new().prefix("test4").tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    env::set_current_dir(&temp_path).unwrap();
    let db = DatabaseOption::MySQL;
    let migrations = PathBuf::from("migrations");
    let seeds = PathBuf::from("seeds");
    assert!(create_config_file(&temp_path, db, migrations, seeds).is_ok());
    assert!(temp_path.join("spectre.toml").exists());
  }
}
