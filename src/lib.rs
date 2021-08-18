use std::io::prelude::*;
use std::io::{self, ErrorKind};
use std::{fs::File, io::Cursor};
use turbo_json_checker::JsonType;

fn enclose_reader(mut reader: impl Read + Seek, start: u64, end: u64) -> anyhow::Result<impl Read> {
    reader.seek(io::SeekFrom::Start(start))?;
    Ok(reader.take(end - start))
}

fn array_reader(
    reader: impl Read + Seek,
    start: u64,
    end: u64,
) -> anyhow::Result<Option<impl Read>> {
    let mut enclosed_reader = enclose_reader(reader, start, end)?;
    dbg!(start, end);
    let mut c = [0];

    loop {
        if let Err(err) = enclosed_reader.read_exact(&mut c) {
            if err.kind() == ErrorKind::UnexpectedEof {
                return Ok(None);
            }
        }
        dbg!(c[0] as char);
        if !(c[0] as char).is_whitespace() {
           break;
        }
    }
    Ok(Some(Cursor::new(c.clone()).chain(enclosed_reader)))
}

pub fn json_combine(file_paths: Vec<String>, mut writer: impl Write) {
    if let Err(error) = writer.write_all(b"[") {
        eprintln!("{}", error);
        panic!("Could not write in the output stream");
    }
    let mut first_element = true;

    for (index, file_path) in file_paths.iter().enumerate() {
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("File: {} could not be opened", file_path);
                eprintln!("{}", error);
                continue;
            }
        };

        let enclosed_reader = match turbo_json_checker::validate(&file) {
            Ok((JsonType::Array, start, end)) => array_reader(file, start as u64 + 1, end as u64)
                .unwrap()
                .map(|o| Box::new(o) as Box<dyn Read>),
            Ok((_, start, end)) => Some(Box::new(
                enclose_reader(file, start as u64, end as u64).unwrap(),
            ) as Box<dyn Read>),
            Err(error) => {
                eprintln!("File {} is not a valid JSON ", file_path);
                eprintln!("{}", error);
                continue;
            }
        };
        dbg!(&enclosed_reader.is_none());
        if enclosed_reader.is_none() {
            continue;
        }
        let mut enclosed_reader = enclosed_reader.unwrap();

        if first_element == false {
            if let Err(error) = writer.write_all(b",") {
                eprintln!("{}", error);
                panic!("Could not write in the output stream");
            }
        } else {
            first_element = false;
        }

        if let Err(error) = io::copy(&mut enclosed_reader, &mut writer) {
            eprintln!("{}", error);
            panic!("Could not read and write from {}", file_path);
        }
    }

    if let Err(error) = writer.write_all(b"]") {
        eprintln!("{}", error);
        panic!("Could not write in the output stream");
    }
}
