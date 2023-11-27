use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Write;
use std::{collections::BTreeMap, fs};
use tera::{Context, Tera};

use crate::{config, StartArgs};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TmuxWindowConfig {
    name: String,
    command: String,
}

// TODO(tatu): Rename to project config
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TmuxScriptTemplate {
    is_new_tmux_session: bool,
    attach: bool,
    windows: Vec<TmuxWindowConfig>,
    root: String,
    name: String,
}

fn write_tmux_template(
    s: &mut dyn Write,
    config: &TmuxScriptTemplate,
) -> Result<(), Box<dyn Error>> {
    // TOOD(tatu): Add proper error handling
    Ok(write!(s, "{}", render_tmux_template(config)?)?)
}

fn render_tmux_template(config: &TmuxScriptTemplate) -> Result<String, Box<dyn Error>> {
    // TOOD(tatu): Add proper error handling
    let mut tera = Tera::default();
    tera.add_raw_template("tmux.sh", include_str!("templates/tmux.sh"))?;
    Ok(tera.render("tmux.sh", &Context::from_serialize(&config)?)?)
}

impl TmuxScriptTemplate {
    fn build(config: Config, runtime_args: &StartArgs) -> Result<Self, Box<dyn Error>> {
        let windows = config
            .windows
            .iter()
            .map(|window_config| TmuxWindowConfig {
                name: window_config.first_key_value().unwrap().0.to_string(),
                command: window_config
                    .first_key_value()
                    .unwrap()
                    .1
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_string(),
            })
            .collect();

        Ok(TmuxScriptTemplate {
            is_new_tmux_session: false,
            attach: !runtime_args.no_attach,
            windows,
            name: config.name,
            root: config.root.unwrap_or(".".to_string()),
        })
    }
}

// TODO(tatu): Add default values
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    name: String,
    root: Option<String>,
    windows: Vec<BTreeMap<String, Option<String>>>,
}

pub struct Project {
    tmux_script_template: TmuxScriptTemplate,
}

impl Project {
    pub fn load(
        config: config::Config,
        options: &StartArgs,
        project_name: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let project_file = config.layout_path.join(project_name).with_extension("yml");

        if project_file.is_file() {
            let contents = fs::read_to_string(project_file)
                .expect("Could not read given project file, check permissions");

            let config: Config = serde_yaml::from_str(&contents).unwrap();

            let tmux_template = TmuxScriptTemplate::build(config, &options)?;

            Ok(Project {
                tmux_script_template: tmux_template,
            })
        } else {
            println!("Given project does not exist or is not a file");
            // TODO(tatu): We should fall to create in this case
            unimplemented!("Loading missing projects not implemented!");
        }
    }

    pub fn render(&self) -> Result<String, Box<dyn Error>> {
        render_tmux_template(&self.tmux_script_template)
    }
}
