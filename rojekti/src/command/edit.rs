use std::fs::File;
use std::io::Write;
use std::process::Command;

use crate::config::Config;
use crate::error::Result;
use crate::project::render_default_template;

pub fn run(config: Config, project_name: &str) -> Result<()> {
    let project_file = config.layout_path.join(project_name).with_extension("yml");

    if !project_file.is_file() {
        let mut file = File::create(&project_file)?;
        file.write_all(
            render_default_template(&project_file, project_name, &config.pwd)?.as_bytes(),
        )?;
    }

    Command::new(config.editor)
        .args([project_file.to_str().ok_or("Not a valid path")?])
        .status()?;

    Ok(())
}
