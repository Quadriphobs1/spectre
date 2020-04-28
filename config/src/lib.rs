#[macro_use]
extern crate serde_derive;
extern crate spectre_connection;

mod config;
mod errors;
mod option;
mod parser;

pub use self::config::Config;
use self::errors::Error;
pub use self::option::ConfigOption;
use self::parser::Parser;

type Result<T> = std::result::Result<T, Error>;
