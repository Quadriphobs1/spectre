#[macro_use]
extern crate serde_derive;
extern crate connection;
extern crate regex;

mod config;
mod errors;
mod logger;
mod option;
mod parser;

pub use self::config::Config;
use self::errors::Error;
use self::logger::{Logger, Logging};
use self::option::ConfigOption;

type Result<T> = std::result::Result<T, Error>;
