use std::fs::File;
use std::io;
use std::io::prelude::*;

/// All JSON types are valid input.
/// Nonetheless, if the JSON is an array it will be flattened in the output
/// Flattening is only one level deep
/// Ex:
/// input: [1, 2, [3]] & "hello"
/// output: [1, 2, [3], "hello"]
fn normalize_json_types(mut file: impl Read, writer: &mut impl Write) -> io::Result<()> {
    let mut buffer = [0; 10];
    let mut first_char_found = false;
    loop {
        let size = file.read(&mut buffer)?;
        if !first_char_found {
            let first_none_white_position =
                &buffer.iter().position(|c| !(*c as char).is_whitespace());
            if let Some(char_position) = first_none_white_position {
                first_char_found = true;
                let start = match &buffer[*char_position] {
                    b'[' => char_position + 1,
                    _ => *char_position,
                };
                writer.write_all(&buffer[start..size])?;
            }
        } else {
            writer.write_all(&buffer[0..size])?;
        }
        if size == 0 {
            break;
        }
    }
    Ok(())
}

pub fn json_combine(file_paths: Vec<String>, mut writer: impl Write) {

    if let Err(error) = writer.write_all(b"[") {
        eprintln!("{}", error);
        panic!("Could not write in the output stream");
    }
    for (index, file_path) in file_paths.iter().enumerate() {
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
        if index != 0 {
            if let Err(error) = writer.write_all(b",") {
                eprintln!("{}", error);
                panic!("Could not write in the output stream");
            }
        }

        if let Err(error) = file.seek(io::SeekFrom::Start(0)) {
            eprintln!("Getting back to the start of file {} failed", file_path);
            eprintln!("{}", error);
            panic!("Getting back to the start of file {} failed", file_path);
        }
        let mut buffer = [0; 10];

        if let Err(error) = normalize_json_types(file, &mut writer) {
            eprintln!("{}", error);
            panic!("could not read file: {}", file_path);
        }
    }

    if let Err(error) = writer.write_all(b"]") {
        eprintln!("{}", error);
        panic!("Could not write in the output stream");
    }
}
