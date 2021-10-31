use crate::validator::validate_files;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use termion::{color, style};

// Posibilite de recursivite et de double path resolver

/// Combines json files in the provided [`Write`]r.
///
/// The files are combined into an array. Only the valid JSON's make the cut.
/// Meaning, a validation is done on each file before it is combined.
///
/// All reading and writing is done in a streaming manner. Resulting in a low memory usage and a fast processing.
/// Since reading is done once for validation and once for writing, seeking is required inside the file. Because of that, it does not support a simple [`Read`](std::io::Read)er.
///
/// If the **JSON file root type is an array**, it will concatenates with the output array.
///
pub fn json_combine(file_paths: Vec<String>, mut writer: impl Write) {
    if let Err(error) = writer.write_all(b"[") {
        eprintln!("{}", error);
        panic!("Could not write in the output stream");
    }
    let mut first_element = true;

    // Create progress bar
    let valid_files_reader = validate_files(&file_paths);
    let number_of_files = valid_files_reader.len() as u64;
    let files_write_progress_bar = ProgressBar::new(number_of_files);

    files_write_progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} combining... [{elapsed_precise}] [{wide_bar:.green}]▏{pos}/{len}",
            ) // green does not work, it is always white
            .progress_chars("█▇▆▅▄▃▂▁  "),
    );

    for valid_files_reader in valid_files_reader.into_iter() {
        let mut file_reader = match valid_files_reader {
            Ok(Some(reader)) => reader,
            Ok(None) => continue,
            Err(error) => {
                files_write_progress_bar.println(error.to_string());
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

        // Increment progress bar by one
        files_write_progress_bar.inc(1);
    }
    files_write_progress_bar.println(format!(
        "{}{}[SUCCESS]{} Valid JSON files have been merged.",
        color::Fg(color::Green),
        style::Bold,
        style::Reset
    ));
    if let Err(error) = writer.write_all(b"]") {
        eprintln!("{}", error);
        panic!("Could not write in the output stream");
    }
}
