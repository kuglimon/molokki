use std::io::{self, Write};

use crate::{config::Config, error::Result, project::Project, StartArgs};

pub fn run(config: Config, args: &StartArgs, project_name: &str) -> Result<()> {
    let project = Project::load(config, args, project_name)?;
    writeln!(io::stdout(), "{}", project.render()?)?;
    Ok(())
}
