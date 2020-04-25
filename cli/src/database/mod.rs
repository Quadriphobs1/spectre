mod command;
mod config;
mod drop;
mod fresh;
mod reset;
mod setup;

pub use self::command::*;
pub use self::config::*;
pub use self::drop::*;
pub use self::fresh::*;
pub use self::reset::*;
pub use self::setup::*;
use crate::result::CliError;
use crate::util::handle_error;
use std::env;

pub fn database_url() -> String {
  env::var("DATABASE_URL").unwrap_or_else(|_| handle_error(CliError::DatabaseUrlMissing))
}
