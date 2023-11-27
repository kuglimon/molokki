use std::error::Error;
use std::fs::DirEntry;
use std::io::{self, Write};
use std::path::Path;
use std::{env, fs};

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

pub fn run(split_by_newline: bool) -> Result<(), Box<dyn Error>> {
    // TODO(tatu): Maybe move this under environment or some similar struct
    let home_path = env::var("HOME").expect("HOME is not set on env, cannot continue");
    // TODO(tatu): Doesn't support .config in another directory, but I never change this, meh.main
    let xdg_config_home = Path::new(&home_path).join(".config");
    let layout_home = xdg_config_home.join("tmuxinator");

    let mut paths = fs::read_dir(&layout_home)?;

    {
        let mut lock = io::stdout().lock();
        let separator = if split_by_newline { "\n" } else { " " };

        if let Some(path) = paths.next() {
            write!(lock, "{}", path_to_filename(path?)?)?;
        };

        for path in paths {
            write!(lock, "{}{}", separator, path_to_filename(path?)?)?
        }

        write!(lock, "\n")?;
    }
    Ok(())
}
