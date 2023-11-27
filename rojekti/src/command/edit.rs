use std::env;
use std::error::Error;
use std::process::Command;

use crate::config::Config;

pub fn run(config: Config, project_name: &str) -> Result<(), Box<dyn Error>> {
    let project_file = config.layout_path.join(project_name).with_extension("yml");

    Command::new(env::var("EDITOR").expect("Broke ass environment does not have EDITOR set"))
        .args([project_file.to_str().ok_or("Not a valid path")?])
        .status()?;
    Ok(())
}
