use std::env::args;


fn main() {
  let mut arguments = args().skip(1);

  if arguments.any( |arg| arg == "--help") {
    println!("json-boat: 0.1.0
    USAGE:
      json-board [files]
    ");
    return;
  }
  let arguments = args().skip(1);


  // ./jsonboat data/*
  // ./jsonboat --help
  // ./jsonboat --version
  dbg!(arguments.collect::<Vec<String>>());
}
