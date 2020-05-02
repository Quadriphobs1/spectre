use std::fmt::Display;

pub fn handle_error<E: Display, T>(error: E) -> T {
  eprintln!("{}", error);
  ::std::process::exit(1);
}
