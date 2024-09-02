use std::fs;
use std::fs::DirEntry;
use std::io::{self, Write};

use crate::config::RuntimeEnvironment;
use crate::error::Result;

// TODO: Does this really have to be this verbose?
fn path_to_filename(path: DirEntry) -> Result<String> {
    Ok(path
        .path()
        .with_extension("")
        .file_name()
        .ok_or("extension error")?
        .to_str()
        .ok_or("osstr error")?
        .to_string())
}

pub fn run(config: RuntimeEnvironment, split_by_newline: bool) -> Result<()> {
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
