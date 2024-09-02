use std::io::{self, Write};

use crate::{config::RuntimeEnvironment, error::Result, project::ProjectState, StartArgs};

pub fn run(config: RuntimeEnvironment, args: &StartArgs, project_name: &str) -> Result<()> {
    let state = ProjectState::load(&config, args, project_name)?;

    match state {
        ProjectState::New(_) => Err("File not found".into()),
        ProjectState::Exists(project) => {
            writeln!(io::stdout(), "{}", project.render()?)?;
            Ok(())
        }
    }
}
