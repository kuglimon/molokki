use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::BTreeMap, fs};
use tera::{Context, Tera};

use crate::error::Result;
use crate::{config, StartArgs};

fn render_tmux_template(config: &Project) -> Result<String> {
    // TOOD(tatu): Add proper error handling
    let mut tera = Tera::default();
    tera.add_raw_template("tmux.sh", include_str!("templates/tmux.sh"))?;
    Ok(tera.render("tmux.sh", &Context::from_serialize(config)?)?)
}

pub fn render_default_template(
    project_file: &std::path::PathBuf,
    project_name: &str,
    pwd: &PathBuf,
) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "sample_config.yml",
        include_str!("templates/sample_config.yml"),
    )?;

    let mut context = Context::new();
    // FIXME: don't unwrap
    context.insert("path", project_file.to_str().unwrap());
    context.insert("name", project_name);
    context.insert("pwd", pwd);

    Ok(tera.render("sample_config.yml", &context)?)
}

pub enum ProjectState {
    New(NewProject),
    Exists(Project),
}

impl ProjectState {
    pub fn load(config: &config::Config, options: &StartArgs, project_name: &str) -> Result<Self> {
        let project_file = config.layout_path.join(project_name).with_extension("yml");

        if project_file.is_file() {
            let contents = fs::read_to_string(project_file)
                .expect("Could not read given project file, check permissions");

            Ok(ProjectState::Exists(Project::load_str(options, &contents)?))
        } else {
            Ok(ProjectState::New(NewProject {}))
        }
    }
}

pub struct NewProject {}

// TODO(tatu): Add default values
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    name: String,
    root: Option<String>,
    windows: Vec<BTreeMap<String, Option<String>>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    root: String,
    name: String,
    attach: bool,
    windows: Vec<Window>,
    is_new_tmux_session: bool,
}

// SinglePanel(Panel),
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PanelConfig {
    SinglePanel(Panel),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Window {
    name: String,
    panels: PanelConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Panel {
    command: Option<String>,
}

impl Project {
    fn load_str(options: &StartArgs, contents: &str) -> Result<Self> {
        let config: Config = serde_yaml::from_str(contents).unwrap();

        let windows = config
            .windows
            .iter()
            .map(|window_config| {
                let raw_command = window_config
                    .first_key_value()
                    .unwrap()
                    .1
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_string();

                let command = match raw_command.as_str() {
                    "nil" => None,
                    c => Some(c.to_string()),
                };

                let panels = PanelConfig::SinglePanel(Panel { command });

                Window {
                    name: window_config.first_key_value().unwrap().0.to_string(),
                    panels,
                }
            })
            .collect();

        Ok(Project {
            is_new_tmux_session: false,
            attach: !options.no_attach,
            windows,
            name: config.name,
            root: config.root.unwrap_or(".".to_string()),
        })
    }

    pub fn render(&self) -> Result<String> {
        render_tmux_template(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::Project;
    use crate::StartArgs;

    #[test]
    fn it_parses_simple_layouts() {
        let yaml = r###"# /home/somebody/.config/tmuxinator/base.yml

            name: PathOfBuilding
            root: /home/somebody/development/personal/PathOfBuilding

            windows:
            - editor: vim -u NONE
            - backend: "docker compose up --build"
            - sandbox: null
            - service: null
            "###;

        let layout_options = StartArgs {
            name: "PathOfBuilding".to_string(),
            no_attach: false,
        };

        let project = Project::load_str(&layout_options, yaml);

        assert!(project.is_ok(), "should be able to load project layout")
    }

    #[test]
    fn it_parses_windows_commands_nil_null_and_empty() {
        let yaml = r###"# /home/somebody/.config/tmuxinator/base.yml

            name: PathOfBuilding
            root: /home/somebody/development/personal/PathOfBuilding

            windows:
            - sandbox: nil
            - service: null
            - another:
            "###;

        let layout_options = StartArgs {
            name: "PathOfBuilding".to_string(),
            no_attach: false,
        };

        let project = Project::load_str(&layout_options, yaml);

        assert!(project.is_ok(), "should be able to load project layout")
    }
}
