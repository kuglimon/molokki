use std::{env, os::unix::process::CommandExt, process::Command};

use crate::error::Result;
use crate::{config::Config, project::Project, StartArgs};

pub fn run(config: Config, args: &StartArgs, project_name: &str) -> Result<()> {
    let project = Project::load(config, args, project_name)?;
    let script = project.render()?;

    // TODO(tatu): Handle errors
    Command::new(env::var("SHELL").expect("SHELL not set, brother get some help"))
        .args(["-c", &script])
        .exec();

    Ok(())
}
