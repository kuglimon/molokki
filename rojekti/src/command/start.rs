use std::{env, error::Error, io, os::unix::process::CommandExt, process::Command};

use crate::{config::Config, project::Project, StartArgs};

pub fn run(config: Config, args: &StartArgs, project_name: &str) -> Result<(), Box<dyn Error>> {
    let project = Project::load(config, args, &args.name)?;
    let script = project.render()?;

    // TODO(tatu): Handle errors
    Command::new(env::var("SHELL").expect("SHELL not set, brother get some help"))
        .args(["-c", &script])
        .exec();

    Ok(())
}
