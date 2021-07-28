use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn json_combine(file_paths: Vec<String>, mut writer: impl Write) {
    for file_path in &file_paths {
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

            writer.write(&buffer[..size]);
            if size == 0 {
                break;
            }
        }
    }
}
