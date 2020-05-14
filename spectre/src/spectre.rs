pub use config::{Config, ConfigManager};
pub use connection::{Connection, ConnectionManager, ConnectionOption, ConnectionOptionManager};
use directory::find_project_root;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Spectre {
  config: Config,
  option_manager: ConnectionOptionManager,
  manager: ConnectionManager,
}

impl Spectre {
  // create a new instance loading the config from root
  pub fn from(root: &Path) -> Spectre {
    Spectre::configured(root.into())
  }

  // create a new spectre using a config from `spectre.yaml` or `spectre.yml`.
  pub fn init() -> Spectre {
    let root = match find_project_root() {
      Ok(path) => path,
      Err(error) => {
        println!("{}", error);
        ::std::process::exit(1);
      }
    };
    Spectre::configured(root)
  }

  pub fn custom(config: Config) -> Spectre {
    let option_manager = ConnectionOptionManager::new(&config.connections);
    let manager = ConnectionManager::new();
    Spectre {
      config,
      option_manager,
      manager,
    }
  }

  // Connect to all connections provided
  pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
    for _connection in self.manager.connections().values() {
      unimplemented!()
      // connection.connect().unwrap();
    }
    Ok(())
  }

  pub fn connect_async() -> Result<(), Box<dyn Error>> {
    Ok(())
  }

  // connect to a particular connection
  pub fn connect_to(&self, name: &str) -> Result<(), Box<dyn Error>> {
    match self.manager().get(name) {
      Some(_connection) => {
        // connection.connect().unwrap();
        Ok(())
      }
      None => panic!("Connection with the provided name"),
    }
  }

  // connect to a particular connection
  pub fn connect_to_async(_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
  }

  // get an instance of a connection
  pub fn get_connection(&self, name: &str) -> Option<&'_ Connection> {
    self.manager.get(name)
  }

  // get options of a connection
  pub fn get_option(&self, name: &str) -> Option<&'_ ConnectionOption> {
    self.option_manager.get(name)
  }

  #[inline(always)]
  pub fn config(&self) -> &Config {
    &self.config
  }

  #[inline(always)]
  pub fn manager(&self) -> &ConnectionManager {
    &self.manager
  }

  #[inline(always)]
  pub fn option_manager(&self) -> &ConnectionOptionManager {
    &self.option_manager
  }

  #[inline]
  fn configured(root: PathBuf) -> Spectre {
    let config = ConfigManager::from(root).unwrap().config();
    let option_manager = ConnectionOptionManager::new(&config.connections);
    let manager = ConnectionManager::new();
    Spectre {
      config,
      option_manager,
      manager,
    }
  }
}
