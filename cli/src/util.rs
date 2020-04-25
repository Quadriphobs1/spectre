use std::fmt::Display;

pub fn handle_error<E: Display, T>(error: E) -> T {
  println!("{}", error);
  ::std::process::exit(1);
}
