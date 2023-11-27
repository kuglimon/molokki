use std::io;

use crate::{config::Config, project::Project, StartArgs};

pub fn run(config: Config, args: &StartArgs, project_name: &str) -> Result<(), Box<dyn Error>> {
    let project = Project::load(config, args, &args.name)?;
    writeln!(io::stdout(), "{}", project.render()?)?;
    Ok(())
}
