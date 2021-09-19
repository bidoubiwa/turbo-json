use rayon::prelude::*;
use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::{fs::File, io::Cursor};
use turbo_json_checker::JsonType;
use std::fmt::Display;

fn enclose_reader(mut reader: impl Read + Seek, start: u64, end: u64) -> io::Result<impl Read> {
    reader.seek(io::SeekFrom::Start(start))?;
    Ok(reader.take(end - start))
}

fn array_reader(reader: impl Read + Seek, start: u64, end: u64) -> io::Result<Option<impl Read>> {
    let mut enclosed_reader = enclose_reader(reader, start, end)?;
    let mut c = [0];

    loop {
        // In case the array is empty, we catch the UnexpectedEof to return a None
        // Meaning there is nothing to flatten from this array.
        if let Err(err) = enclosed_reader.read_exact(&mut c) {
            if err.kind() == ErrorKind::UnexpectedEof {
                return Ok(None);
            }
        }
        if !(c[0] as char).is_whitespace() {
            // In this case, we found elements in the array
            // They will be flatten in the output JSON array
            break;
        }
    }

    // In case we found an element in the array
    // We create a new Reader starting on the first caracter followed by
    // the rest of the reader.
    // Example c[0] = `"` in the case of [     "a", "b", "c"]
    // enclosed_reader has its cursor placed on `a`
    // We chain a reader starting at c[0] with the cursor position of enclosed_reader
    Ok(Some(Cursor::new(c.clone()).chain(enclosed_reader)))
}

enum JSONFileError {
    IOError(String, std::io::Error),
    InvalidJSON(String, std::io::Error),
}

impl Display for JSONFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JSONFileError::IOError(message, error) => {
                write!(f, "{} \n {}", message, error)
            }
            JSONFileError::InvalidJSON(message, error) => {
                write!(f, "{} \n  {}", message, error)
            }
        }
    }
}

fn validate_files(files_path: &[String]) -> Vec<Result<Option<Box<dyn Read + Send>>, JSONFileError>> {
    files_path
        .par_iter()
        .map(|file_path| {
            let file = match File::open(file_path) {
                Ok(file) => file,
                Err(error) => {
                    let message = format!("File: {} could not be opened", file_path);
                    return Err(JSONFileError::IOError(message, error));
                }
            };
            match turbo_json_checker::validate(&file) {
                Ok((JsonType::Array, start, end)) => {
                    match array_reader(file, start as u64 + 1, end as u64) {
                        Ok(cursor_option) => {
                            Ok(cursor_option.map(|reader| Box::new(reader) as Box<dyn Read + Send>))
                        }
                        Err(error) => Err(JSONFileError::IOError(String::new(), error)),
                    }
                }
                Ok((_, start, end)) => {
                    match enclose_reader(file, start as u64, end as u64 + 1) {
                        Ok(cursor) => {
                            Ok(Some(Box::new(cursor) as Box<dyn Read  + Send>))
                        },
                        Err(error) => Err(JSONFileError::IOError(String::new(), error)),
                }},
                Err(error) => {
                    let message = format!("File {} is not a valid JSON", file_path);
                    Err(JSONFileError::InvalidJSON(message, error))
                }
            }
        })
        .collect()
}

pub fn json_combine(file_paths: Vec<String>, mut writer: impl Write) {
    if let Err(error) = writer.write_all(b"[") {
        eprintln!("{}", error);
        panic!("Could not write in the output stream");
    }
    let mut first_element = true;

    let valid_files_reader = validate_files(&file_paths);

    for valid_files_reader in valid_files_reader.into_iter() {
        let mut file_reader = match valid_files_reader {
            Ok(Some(reader)) => reader,
            Ok(None) => continue,
            Err(error) => {
                eprintln!("{}", error);
                continue
            }
        };

        if first_element == false {
            if let Err(error) = writer.write_all(b",") {
                eprintln!("{}", error);
                panic!("Could not write in the output stream");
            }
        } else {
            first_element = false;
        }

        if let Err(error) = io::copy(&mut file_reader, &mut writer) {
            panic!("{}", error);
        }
    }

    if let Err(error) = writer.write_all(b"]") {
        eprintln!("{}", error);
        panic!("Could not write in the output stream");
    }
}
