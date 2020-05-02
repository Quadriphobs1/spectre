#[macro_use]
extern crate serde_derive;
extern crate connection;
extern crate regex;

mod config;
mod errors;
mod option;
mod parser;

pub use self::config::Config;
use self::errors::Error;
pub use self::option::ConfigOption;

type Result<T> = std::result::Result<T, Error>;
