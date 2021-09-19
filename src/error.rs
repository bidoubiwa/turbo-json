use std::fmt::Display;
use termion::{color, style};

impl Display for JSONFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JSONFileError::IOError(error, file_path) => {
                write!(
                    f,
                    "{}{}[Input/Output Error]{} {}. Could not read/write: {} {} {}",
                    color::Fg(color::Red),
                    style::Bold,
                    style::Reset,
                    error,
                    color::Fg(color::Cyan),
                    file_path,
                    style::Reset
                )
            }
            JSONFileError::InvalidJSON(error, file_path) => {
                write!(
                    f,
                    "{}{}[Invalid JSON]{} {}. Could not read/write: {} {} {}",
                    color::Fg(color::Yellow),
                    style::Bold,
                    style::Reset,
                    error,
                    color::Fg(color::Cyan),
                    file_path,
                    style::Reset
                )
            }
        }
    }
}

pub enum JSONFileError {
    IOError(std::io::Error, String),
    InvalidJSON(std::io::Error, String),
}
