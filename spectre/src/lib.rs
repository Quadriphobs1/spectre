extern crate config;
extern crate connection;
extern crate directory;

pub use config::Config;
use directory::find_project_root;
use std::error::Error;
use std::path::Path;

#[derive(Default, Debug)]
pub struct Spectre {
  config: Config,
}

impl Spectre {
  // create a new instance loading the config from root
  pub fn from(root: &Path) -> Spectre {
    let config = Config::from(root);
    Spectre {
      config: config.unwrap(),
    }
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

    let config = Config::from(root);

    Spectre {
      config: config.unwrap(),
    }
  }

  // Connect to all connections provided
  pub fn connect() -> Result<(), Box<dyn Error>> {
    Ok(())
  }

  // connect to a particular connection
  pub fn connect_to(_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
  }

  // get an instance of a connection
  pub fn get_connection(_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
  }

  // get options of a connection
  pub fn get_connection_option(_name: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
  }
}
