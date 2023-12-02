use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::io::{self, Write};

use crate::config::Config;

// TODO: Does this really have to be this verbose?
fn path_to_filename(path: DirEntry) -> Result<String, Box<dyn Error>> {
    Ok(path
        .path()
        .with_extension("")
        .file_name()
        .ok_or("extension error")?
        .to_str()
        .ok_or("osstr error")?
        .to_string())
}

pub fn run(config: Config, split_by_newline: bool) -> Result<(), Box<dyn Error>> {
    let mut paths = fs::read_dir(&config.layout_path)?;

    let separator = if split_by_newline { "\n" } else { " " };

    if let Some(path) = paths.next() {
        write!(io::stdout(), "{}", path_to_filename(path?)?)?;
    };

    for path in paths {
        write!(io::stdout(), "{}{}", separator, path_to_filename(path?)?)?
    }

    writeln!(io::stdout())?;
    Ok(())
}
