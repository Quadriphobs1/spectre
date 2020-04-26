#[macro_use]
extern crate serde_derive;
extern crate spectre_connection;
extern crate toml;

mod config;
mod directory;
mod errors;
mod option;
mod parser;

pub use self::config::Config;
pub use self::errors::Error;
pub use self::option::ConfigOption;
pub use self::parser::Parser;

pub type Result<T> = std::result::Result<T, Error>;
