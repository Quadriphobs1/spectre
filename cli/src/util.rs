use std::fmt::Display;

pub fn handle_error<E: Display, T>(error: E) -> T {
  // TODO: Gracefully handle this error
  panic!("{}", error);
  // eprintln!("{}", error);
  // ::std::process::exit(1);
}
