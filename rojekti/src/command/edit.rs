use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use crate::config::Config;
use crate::project::render_default_template;

pub fn run(config: Config, project_name: &str) -> Result<(), Box<dyn Error>> {
    let project_file = config.layout_path.join(project_name).with_extension("yml");

    if !project_file.is_file() {
        let mut file = File::create(&project_file)?;
        file.write_all(render_default_template(&project_file, project_name)?.as_bytes())?;
    }

    Command::new(config.editor)
        .args([project_file.to_str().ok_or("Not a valid path")?])
        .status()?;

    Ok(())
}
