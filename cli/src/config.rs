use crate::database::DatabaseConfig;
use crate::directory::find_project_root;
use crate::result::{CliError, CliResult};
use crate::util::handle_error;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Default, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
  #[serde(default)]
  pub database: DatabaseConfig,
  #[serde(default)]
  pub seeds: Option<SeedsDirectory>,
  #[serde(default)]
  pub migrations: Option<MigrationsDirectory>,
}

impl Config {
  pub fn load() -> Result<Self, Box<dyn Error>> {
    let root = find_project_root()?;
    let path = Path::new(&root).join("spectre.toml");

    if path.exists() {
      let mut bytes = Vec::new();
      fs::File::open(&path)?.read_to_end(&mut bytes)?;
      let mut result = toml::from_slice::<Self>(&bytes)?;
      result.set_relative_path_base(path.parent().unwrap());
      result.verify_relative_path_exist()?;
      Ok(result)
    } else {
      Ok(Self::default())
    }
  }

  fn verify_relative_path_exist(&mut self) -> CliResult<()> {
    if let Some(ref mut migration) = self.migrations {
      migration.verify_relative_path_exist()?;
    }
    if let Some(ref mut seeds) = self.seeds {
      seeds.verify_relative_path_exist()?;
    }
    Ok(())
  }

  fn set_relative_path_base(&mut self, base: &Path) {
    if let Some(ref mut migration) = self.migrations {
      migration.set_relative_path_base(base);
    }
    if let Some(ref mut seeds) = self.seeds {
      seeds.set_relative_path_base(base);
    }
  }
}

#[derive(Default, Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MigrationsDirectory {
  pub dir: PathBuf,
}

impl MigrationsDirectory {
  fn set_relative_path_base(&mut self, base: &Path) {
    if self.dir.is_relative() {
      self.dir = base.join(&self.dir);
    }
  }

  fn verify_relative_path_exist(&self) -> CliResult<()> {
    if !self.dir.is_dir() {
      handle_error(CliError::MigrationsDirectoryNotFound(self.dir.clone()))
    }
    Ok(())
  }
}

#[derive(Default, Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SeedsDirectory {
  pub dir: PathBuf,
}

impl SeedsDirectory {
  fn set_relative_path_base(&mut self, base: &Path) {
    if self.dir.is_relative() {
      self.dir = base.join(&self.dir);
    }
  }

  fn verify_relative_path_exist(&self) -> CliResult<()> {
    if !self.dir.is_dir() {
      handle_error(CliError::SeedsDirectoryNotFound(self.dir.clone()))
    }
    Ok(())
  }
}
