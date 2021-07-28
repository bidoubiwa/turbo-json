use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::env::args;
use std::str;
use json_boat::json_boat;

fn main() -> io::Result<()> {
  let arguments: Vec<_> = args().skip(1).collect();

  if arguments.iter().any( |arg| arg == "--help") || arguments.is_empty() {
    println!("json-boat: 0.1.0
    USAGE:
      json-board [files]
    ");
    return Ok(());
  }

  let stdout = io::stdout();
  let stdout = stdout.lock();

  json_boat(arguments, stdout);
  // dbg!(arguments);
  Ok(())
}
