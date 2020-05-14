#[macro_use]
extern crate serde_derive;

mod connection;
mod datasource;
mod driver;
mod errors;
mod manager;
mod option;

use self::driver::{create_driver, Driver};
use crate::errors::Error;
use std::result;

pub use self::connection::Connection;
pub use self::datasource::{Datasource, Provider};
pub use self::manager::ConnectionManager;
pub use self::option::{ConnectionOption, ConnectionOptionManager};
pub type Result<T> = result::Result<T, Error>;
