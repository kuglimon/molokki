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
    pub fn load(config: &config::RuntimeEnvironment, options: &StartArgs, project_name: &str) -> Result<Self> {
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
pub struct Config {
    name: String,
    root: Option<String>,
    socket_name: Option<String>,
    on_project_start: Option<String>,
    on_project_first_start: Option<String>,
    on_project_restart: Option<String>,
    on_project_exit: Option<String>,
    on_project_stop: Option<String>,
    pre_window: Option<String>,
    tmux_options: Option<String>,
    tmux_command: Option<String>,
    startup_window: Option<String>,
    startup_pane: Option<u64>,

    #[serde(default)]
    attach: bool,

    #[serde(default)]
    enable_pane_titles: bool,
    pane_title_position: Option<String>,
    pane_title_format: Option<String>,
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
                    c if c.is_empty() => None,
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
    use super::{Config, Project};
    use crate::{project::PanelConfig, StartArgs};

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
    fn it_parses_empty_window_command() {
        let yaml = r###"# /home/somebody/.config/tmuxinator/base.yml

            name: PathOfBuilding
            root: /home/somebody/development/personal/PathOfBuilding

            windows:
            - another:
            "###;

        let layout_options = StartArgs {
            name: "PathOfBuilding".to_string(),
            no_attach: false,
        };

        let project = Project::load_str(&layout_options, yaml);

        assert!(project.is_ok(), "should be able to load project layout");

        let project = project.unwrap();

        for window in &project.windows {
            match &window.panels {
                PanelConfig::SinglePanel(config) => {
                    assert!(config.command.is_none(), "should be an empty command")
                }
            }
        }
    }

    // This is a breaking change from tmuxinator where these are treated as empty commands, due to
    // being values in ruby. These can be valid commands, nix LSP has a binary called 'nil'.
    // FIXME(tatu): Whole parsing fails with two commands
    #[test]
    fn it_parses_window_commands_nil_and_null_as_commands() {
        let yaml = r###"# /home/somebody/.config/tmuxinator/base.yml

            name: PathOfBuilding
            root: /home/somebody/development/personal/PathOfBuilding

            windows:
            - another: nil
            "###;

        let layout_options = StartArgs {
            name: "PathOfBuilding".to_string(),
            no_attach: false,
        };

        let project = Project::load_str(&layout_options, yaml);

        assert!(project.is_ok(), "should be able to load project layout");

        let project = project.unwrap();

        for window in &project.windows {
            match &window.panels {
                PanelConfig::SinglePanel(config) => {
                    dbg!(config);
                    assert!(config.command.is_some(), "should not be an empty command")
                }
            }
        }
    }

    #[test]
    fn it_parses_full_layout() {
        let yaml = r###"# /home/somebody/.config/tmuxinator/base.yml

            name: sample
            root: ~/

            # Optional tmux socket
            socket_name: foo

            # Note that the pre and post options have been deprecated and will be replaced by
            # project hooks.

            # Project hooks

            # Runs on project start, always
            on_project_start: command-project-start

            # Run on project start, the first time
            on_project_first_start: command-project-first-start

            # Run on project start, after the first time
            on_project_restart: command-on-project-restart

            # Run on project exit ( detaching from tmux session )
            on_project_exit: command-on-project-exit

            # Run on project stop
            on_project_stop: command-on-project-stop

            # Runs in each window and pane before window/pane specific commands. Useful for setting up interpreter versions.
            pre_window: rbenv shell 2.0.0-p247

            # Pass command line options to tmux. Useful for specifying a different tmux.conf.
            tmux_options: -f ~/.tmux.mac.conf

            # Change the command to call tmux. This can be used by derivatives/wrappers like byobu.
            tmux_command: byobu

            # Specifies (by name or index) which window will be selected on project startup. If not set, the first window is used.
            startup_window: editor

            # Specifies (by index) which pane of the specified window will be selected on project startup. If not set, the first pane is used.
            startup_pane: 1

            # Controls whether the tmux session should be attached to automatically. Defaults to true.
            attach: false

            # Enables the display of pane titles. For example "editor" below. Defaults to false.
            enable_pane_titles: true

            # Configures pane title position. Can be: bottom, top, or "off". Note: "off" must be provided in quotes to avoid being interpreted as a boolean false. Defaults to top.
            pane_title_position: bottom

            # Configures pane title format. Defaults to "#{pane_index}: #{pane_title}".
            # Please see the tmux manpage for details, on valid formats.
            pane_title_format: " [ #T ] "

            windows:
            - editor:
                # layout: main-vertical
                # Synchronize all panes of this window, can be enabled before or after the pane commands run.
                # 'before' represents legacy functionality and will be deprecated in a future release, in favour of 'after'
                # synchronize: after
                # panes:
                   # - editor:
                   # - vim
                   # - guard
            - server: bundle exec rails s
            - logs: tail -f log/development.log
            "###;

        let config: Result<Config, _> = serde_yaml::from_str(yaml);

        assert!(config.is_ok(), "should be able to parse config yaml");

        let config = config.unwrap();

        assert!(config.name == "sample");
        assert!(config.root == Some("~/".to_string()));
        assert!(config.socket_name == Some("foo".to_string()));
        assert!(config.on_project_start == Some("command-project-start".to_string()));
        assert!(config.on_project_first_start == Some("command-project-first-start".to_string()));
        assert!(config.on_project_restart == Some("command-on-project-restart".to_string()));
        assert!(config.on_project_exit == Some("command-on-project-exit".to_string()));
        assert!(config.on_project_stop == Some("command-on-project-stop".to_string()));
        assert!(config.pre_window == Some("rbenv shell 2.0.0-p247".to_string()));
        assert!(config.tmux_options == Some("-f ~/.tmux.mac.conf".to_string()));
        assert!(config.tmux_command == Some("byobu".to_string()));
        assert!(config.startup_window == Some("editor".to_string()));
        assert!(config.startup_pane == Some(1));
        assert!(!config.attach);
        assert!(config.enable_pane_titles);
        assert!(config.pane_title_position == Some("bottom".to_string()));
        assert!(config.pane_title_format == Some(" [ #T ] ".to_string()));
    }
}
