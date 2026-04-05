use std::fs::File;
use std::io::Write;
use std::{env, os::unix::process::CommandExt, process::Command};

use crate::error::{Result, RojektiError};
use crate::project::{render_default_template, ProjectState};
use crate::{config::RuntimeEnvironment, StartArgs};

pub fn run(config: RuntimeEnvironment, args: &StartArgs, project_name: &str) -> Result<()> {
    let state = ProjectState::load(&config, args, project_name)?;

    match state {
        ProjectState::New(_) => {
            // FIXME(tatu): Copy-pasta from edit.rs
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
        ProjectState::Exists(project) => {
            let script = project.render()?;

            let shell = env::var("SHELL").map_err(|_| {
                RojektiError::RuntimeError("$SHELL not set correctly, cannot continue".to_owned())
            })?;

            // Exec replaces the project, it can only error out if it returns.
            let status = Command::new(shell).args(["-c", &script]).exec();
            Err(RojektiError::Io(status))
        }
    }
}
