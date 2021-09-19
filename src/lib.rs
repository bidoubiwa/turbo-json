use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use rayon::prelude::*;
use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::{fs::File, io::Cursor};
use turbo_json_checker::JsonType;
use std::fmt::Display;
use termion::{color, style};


enum JSONFileError {
    IOError(std::io::Error, String),
    InvalidJSON(std::io::Error, String)
}

impl Display for JSONFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JSONFileError::IOError(error, file_path) => {
                write!(f, "{}[Input/Output Error]{} {} on {} {} {}", color::Fg(color::Red), style::Reset, error, color::Fg(color::Red), file_path, style::Reset)
            }
            JSONFileError::InvalidJSON(error, file_path) => {
                write!(f, "{}[Invalid JSON Error]{} {} on {} {} {}", color::Fg(color::Red), style::Reset, error, color::Fg(color::Cyan), file_path, style::Reset)
            }
        }
    }
}


fn trim_reader_to_content(mut reader: impl Read + Seek, start: u64, end: u64) -> io::Result<impl Read> {
    reader.seek(io::SeekFrom::Start(start))?;
    Ok(reader.take(end - start))
}

fn flatten_reader_on_array(reader: impl Read + Seek, start: u64, end: u64) -> io::Result<Option<impl Read>> {
    let mut trimmed_reader = trim_reader_to_content(reader, start, end)?;
    let mut c = [0];

    loop {
        // In case the array is empty, we catch the UnexpectedEof to return a None
        // Meaning there is nothing to flatten from this array.
        if let Err(err) = trimmed_reader.read_exact(&mut c) {
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
    Ok(Some(Cursor::new(c.clone()).chain(trimmed_reader)))
}


fn validate_files(
    files_path: &[String],
) -> Vec<Result<Option<Box<dyn Read + Send>>, JSONFileError>> {
    files_path
        .par_iter()
        .progress_count(files_path.len() as u64)
        .map(|file_path| {
            let file = match File::open(file_path) {
                Ok(file) => file,
                Err(error) => {
                    return Err(JSONFileError::IOError(error, file_path.to_string()));
                }
            };
            match turbo_json_checker::validate(&file) {
                Ok((JsonType::Array, start, end)) => {
                    match flatten_reader_on_array(file, start as u64 + 1, end as u64) {
                        Ok(cursor_option) => Ok(
                            cursor_option.map(|reader| Box::new(reader) as Box<dyn Read + Send>)
                        ),
                        Err(error) => {
                            Err(JSONFileError::IOError(error, file_path.to_string())
                        )},
                    }
                }
                Ok((_, start, end)) => match trim_reader_to_content(file, start as u64, end as u64 + 1) {
                    Ok(cursor) => Ok(Some(Box::new(cursor) as Box<dyn Read + Send>)),
                    Err(error) => Err(JSONFileError::IOError(error, file_path.to_string()))
                },
                Err(error) => {
                    Err(JSONFileError::InvalidJSON(error, file_path.to_string()))
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
    let mut error_messages = Vec::new();
    let mut first_element = true;

    let valid_files_reader = validate_files(&file_paths);
    let number_of_files = valid_files_reader.len() as u64;
    for valid_files_reader in valid_files_reader
        .into_iter()
        .progress_count(number_of_files)
    {
        let mut file_reader = match valid_files_reader {
            Ok(Some(reader)) => reader,
            Ok(None) => continue,
            Err(error) => {
                error_messages.push(error);
                continue;
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
    error_messages.iter().for_each(|error| eprintln!("{}", error))
}
