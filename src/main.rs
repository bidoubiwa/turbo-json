use std::env::args;
use std::io;
mod combiner;
mod error;
mod validator;
use combiner::json_combine;

fn main() -> io::Result<()> {
    let arguments: Vec<_> = args().skip(1).collect();

    if arguments.iter().any(|arg| arg == "--help") || arguments.is_empty() {
        println!(
            "turbo-json: 0.1.0
    USAGE:
      turbo-json [files ...]
    "
        );
        return Ok(());
    }

    let stdout = io::stdout();
    let stdout = stdout.lock();

    json_combine(&arguments, stdout);
    Ok(())
}
