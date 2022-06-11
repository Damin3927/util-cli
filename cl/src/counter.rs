use crate::Cli;
use std::{
    error::Error,
    fs,
    io::{BufRead, BufReader, Error as IOError, ErrorKind},
    path::Path,
};

const LINE_ENDING: &'static u8 = &b'\n';

pub fn count_line(cli: &Cli) -> Result<usize, Box<dyn Error>> {
    count_entries_lines(
        Path::new(cli.directory.as_str())
            .canonicalize()?
            .as_os_str()
            .to_str()
            .ok_or(IOError::from(ErrorKind::Other))?,
        &cli.extension,
        cli.verbose,
    )
}

fn count_entries_lines(
    full_path: &str,
    extension: &Option<String>,
    verbosity: bool,
) -> Result<usize, Box<dyn Error>> {
    let mut lines: usize = 0;

    for dir_entry in fs::read_dir(full_path)? {
        let dir_entry = &dir_entry?;
        let dir_entry_path = dir_entry
            .path()
            .to_str()
            .ok_or(IOError::from(ErrorKind::Other))?
            .to_string();

        if dir_entry.file_type()?.is_dir() {
            lines += count_entries_lines(dir_entry_path.as_str(), extension, verbosity)?;
        } else if dir_entry.file_type()?.is_file() {
            lines += count_file_lines(dir_entry_path.as_str(), extension, verbosity)?;
        }
    }

    Ok(lines)
}

fn count_file_lines(
    full_path: &str,
    extension: &Option<String>,
    verbosity: bool,
) -> Result<usize, Box<dyn Error>> {
    if let Some(extension) = extension {
        if !full_path.ends_with(extension) {
            return Ok(0);
        }
    }

    if verbosity {
        println!("{}", full_path);
    }

    let file = std::fs::File::open(full_path)?;
    let mut reader = BufReader::new(file);
    let mut count: usize = 0;
    let mut line: Vec<u8> = Vec::new();

    while match reader.read_until(*LINE_ENDING, &mut line) {
        Ok(bytes) if bytes > 0 => true,
        Err(err) => return Err(Box::new(err)),
        _ => false,
    } {
        match line.last() {
            Some(last) if *last == *LINE_ENDING => count += 1,
            Some(_) => (),
            None => (),
        }
    }

    Ok(count)
}
