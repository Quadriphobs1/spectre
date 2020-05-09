#[macro_use]
extern crate serde_derive;
extern crate connection;
extern crate regex;

mod config;
mod errors;
mod logger;
mod manager;
mod parser;

pub use self::config::Config;
use self::errors::Error;
use self::logger::{Logger, Logging};
pub use self::manager::ConfigManager;

type Result<T> = std::result::Result<T, Error>;
