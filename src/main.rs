use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::env::args;
use std::str;

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
  let mut stdout = stdout.lock();

  for file_path in &arguments {
    let mut file = match File::open(file_path) {
      Ok(file) => file,
      Err(error) => {
        eprintln!("File: {} could not be opened", file_path);
        eprintln!("{}", error);
        continue;
      }
    };

    if let Err(error) = oxidized_json_checker::validate(&file) {
      eprintln!("File {} is not a valid JSON ", file_path);
      eprintln!("{}", error);
      continue;
    }

    if let Err(error) = file.seek(io::SeekFrom::Start(0)) {
      eprintln!("Getting back to the start of file {} failed", file_path);
      eprintln!("{}", error);
      continue;
    }
    let mut buffer = [0; 10];

    loop {
      let size = match file.read(&mut buffer) {
        Ok(size) => size,
        Err(error) => {
          eprintln!("could not read file: {}", file_path);
          eprintln!("{}", error);
          break;
        }
      };

      stdout.write(&buffer[..size]);
      // dbg!(size);
      // dbg!(str::from_utf8(&buffer[..size]));
      if size == 0 {
        break
      }
    }
  }
  dbg!(arguments);
  Ok(())
}
