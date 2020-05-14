extern crate config;
extern crate connection;
extern crate directory;

mod spectre;

pub use self::spectre::Spectre;
pub use config::Config;
pub use connection::*;
use std::path::Path;

/// Alias to [`Spectre::init()`] Creates a new instance of `Spectre`.
pub fn init() -> Spectre {
  Spectre::init()
}

/// Alias to [`Spectre::from()`] Creates a new instance of `Spectre`.
pub fn init_from(root: &Path) -> Spectre {
  Spectre::from(root)
}

/// Alias to [`Spectre::custom()`] Creates a new instance of `Spectre`.
pub fn custom(config: Config) -> Spectre {
  Spectre::custom(config)
}
