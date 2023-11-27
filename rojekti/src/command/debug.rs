use std::error::Error;
use std::io::{self, Write};

use crate::{config::Config, project::Project, StartArgs};

pub fn run(config: Config, args: &StartArgs, project_name: &str) -> Result<(), Box<dyn Error>> {
    let project = Project::load(config, args, project_name)?;
    writeln!(io::stdout(), "{}", project.render()?)?;
    Ok(())
}
