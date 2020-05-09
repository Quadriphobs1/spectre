use crate::directory::find_project_root;
use crate::util::handle_error;
use clap::ArgMatches;
use config::ConfigManager;
use connection::Provider;
use std::{error::Error, fs, path::PathBuf, str::FromStr};

pub fn init_command(matches: &ArgMatches) {
  let base_path = match find_project_root() {
    Ok(path) => path,
    Err(error) => handle_error(error),
  };
  let name = matches.value_of("name");
  let docker = matches.is_present("docker");
  let mut config = ConfigManager::new();

  if let Some(name) = name {
    config.set_config_name(&name);
  }

  config.create(&base_path).unwrap_or_else(handle_error);

  if docker {
    let containers = matches
      .values_of("containers")
      .unwrap()
      .collect::<Vec<&str>>();

    create_docker_compose_file(&base_path, containers).unwrap_or_else(handle_error);
  }
  println!(
    "Project created inside {:?} directory.",
    base_path.to_str().unwrap()
  );
  if docker {
    println!(
      "Docker file has been created, you can now run `docker-compose up` to setup your database with docker.",
    );
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DockerCompose {
  #[serde(default)]
  pub version: String,
  #[serde(default)]
  pub services: Vec<Vec<u8>>,
}

pub fn create_docker_compose_file(
  base: &PathBuf,
  containers: Vec<&str>,
) -> Result<(), Box<dyn Error>> {
  use std::io::Write;

  let path = base.join("docker-compose.yml");
  if !path.exists() {
    let mut file = fs::File::create(path)?;
    let mut docker_template = DockerCompose {
      version: "3.8".to_string(),
      services: Vec::new(),
    };

    for s in containers.into_iter() {
      let provider = Provider::from_str(s)?;
      let template = get_docker_compose_template(&provider).to_vec();
      docker_template.services.push(template);
    }
    let docker_template_str = serde_yaml::to_string(&docker_template)?;
    file.write_all(&docker_template_str.into_bytes())?;
  } else {
    return Err("A config file exist with the name".into());
  }

  Ok(())
}

fn get_docker_compose_template(provider: &Provider) -> &[u8] {
  match provider {
    Provider::MySQL => {
      b"
  mysql:
    image: mysql:alpine
    ports:
      - 3306:3306
    environment:
      MYSQL_ROOT_PASSWORD: admin
      MYSQL_USER: test
      MYSQL_PASSWORD: test
      MYSQL_DATABASE: test
    "
    }
    Provider::Postgres => {
      b"
  postgres:
    image: postgres:alpine
    ports:
      - 5432:5432
    environment:
      POSTGRES_USER: test
      POSTGRES_PASSWORD: test
      POSTGRES_DB: test
    "
    }
    Provider::SQLite => b"",
  }
}

#[cfg(test)]
mod tests {
  extern crate tempfile;

  use super::{create_docker_compose_file, init_command};
  use crate::cli::init_subcommand;
  use clap::App;
  use std::{env, fs, panic, path::PathBuf};
  use tempfile::tempdir;

  #[test]
  fn creates_init_file() {
    let cur_dir = env::current_dir().unwrap();
    let base_path = PathBuf::from(&cur_dir).join("tests/default_init");
    fs::create_dir(&base_path).unwrap();
    env::set_current_dir(&base_path).unwrap();
    fs::File::create(&base_path.join("Cargo.toml")).unwrap();
    let matches = App::new("init-test")
      .subcommand(init_subcommand())
      .get_matches_from(vec!["", "init"]);
    init_command(&matches.subcommand_matches("init").unwrap());
    assert!(&base_path.join("spectre.yaml").exists());
    fs::remove_dir_all(&base_path).unwrap();
    env::set_current_dir(&cur_dir).unwrap();
  }

  // TODO: Cannot run more test here because of the use of env::current_dir() would have been set when the test invokes

  // #[test]
  // fn init_with_name() {
  //   let cur_dir = env::current_dir().unwrap();
  //   let base_path = PathBuf::from(env::current_dir().unwrap()).join("tests/init_with_name");
  //   fs::create_dir(&base_path).unwrap();
  //   env::set_current_dir(&base_path).unwrap();
  //   fs::File::create(&base_path.join("Cargo.toml")).unwrap();
  //   let matches = App::new("init-test")
  //     .subcommand(init_subcommand())
  //     .get_matches_from(vec!["", "init", "-n", "sample"]);

  //   init_command(&matches.subcommand_matches("init").unwrap());
  //   assert!(&base_path.join("sample.yaml").exists());
  //   fs::remove_dir_all(&base_path).unwrap();
  //   env::set_current_dir(&cur_dir).unwrap();
  // }

  // #[test]
  // fn test_init_wih_docker() {
  //   let cur_dir = env::current_dir().unwrap();
  //   let dir = tempdir().unwrap();
  //   let temp_path = dir.path();
  //   env::set_current_dir(&temp_path).unwrap();
  //   let toml_path = temp_path.join("Cargo.toml");

  //   fs::File::create(&toml_path).unwrap();

  //   let matches = App::new("init-test")
  //     .subcommand(init_subcommand())
  //     .get_matches_from(vec![
  //       "",
  //       "init",
  //       "-n",
  //       "sample-docker",
  //       "--containers",
  //       "postgres",
  //       "--docker",
  //     ]);

  //   init_command(&matches.subcommand_matches("init").unwrap());
  //   assert!(temp_path.join("docker-compose.yml").exists());
  //   drop(temp_path.join("Cargo.toml"));
  //   drop(temp_path.join("docker-compose.yml"));
  //   drop(temp_path.join("sample-docker.yml"));
  //   env::set_current_dir(&cur_dir).unwrap();
  //   dir.close().unwrap();
  // }

  // #[test]
  // fn init_docker_error() {
  //   let cur_dir = env::current_dir().unwrap();
  //   let dir = tempdir().unwrap();
  //   let temp_path = dir.path();
  //   env::set_current_dir(&temp_path).unwrap();
  //   let toml_path = temp_path.join("Cargo.toml");
  //   let docker_compose_path = temp_path.join("docker-compose.yml");

  //   fs::File::create(&toml_path).unwrap();
  //   fs::File::create(&docker_compose_path).unwrap();

  //   let matches = App::new("init-test-error")
  //     .subcommand(init_subcommand())
  //     .get_matches_from(vec!["", "init", "--containers", "postgres", "--docker"]);

  //   let result = panic::catch_unwind(|| {
  //     init_command(&matches.subcommand_matches("init").unwrap());
  //   });

  //   assert!(result.is_err());
  //   drop(temp_path.join("Cargo.toml"));
  //   drop(temp_path.join("docker-compose.yml"));
  //   env::set_current_dir(&cur_dir).unwrap();
  //   dir.close().unwrap();
  // }

  #[test]
  fn should_create_docker_compose_file() {
    let dir = tempdir().unwrap();
    let temp_path = dir.path();
    let dbs = vec!["postgres", "mysql", "sqlite"];
    assert!(create_docker_compose_file(&temp_path.into(), dbs).is_ok());
    assert!(temp_path.join("docker-compose.yml").exists());
    dir.close().unwrap();
  }

  #[test]
  fn error_docker_compose_file() {
    let dir = tempdir().unwrap();
    let temp_path = dir.path();
    let docker_file_path = temp_path.join("docker-compose.yml");

    fs::File::create(&docker_file_path).unwrap();
    let dbs = vec!["postgres", "mysql", "sqlite"];
    assert!(create_docker_compose_file(&temp_path.into(), dbs).is_err());
    dir.close().unwrap();
  }
}
