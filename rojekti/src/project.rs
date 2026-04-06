use core::fmt;
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, result};
use yaml_rust2::{Yaml, YamlLoader};

use crate::error::Result;
use crate::tmux::TmuxSessionState;
use crate::{config, StartArgs};

fn render_tmux_template(config: &Project) -> Result<String> {
    // TOOD(tatu): Add proper error handling
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.add_template("tmux.sh.jinja", include_str!("templates/tmux.sh.jinja"))
        .unwrap();
    let template = env.get_template("tmux.sh.jinja").unwrap();

    Ok(template.render(config).unwrap())
    // tera.add_raw_template("tmux.sh.tera")?;
    // Ok(tera.render("tmux.sh.tera", &Context::from_serialize(config)?)?)
}

pub fn render_default_template(
    project_file: &std::path::PathBuf,
    project_name: &str,
    pwd: &PathBuf,
) -> Result<String> {
    let mut env = Environment::new();
    env.add_template(
        "sample_config.yml",
        include_str!("templates/sample_config.yml"),
    )
    .unwrap();
    let template = env.get_template("sample_config.yml").unwrap();

    let context = context! {
        path => project_file.to_str().unwrap(),
        name => project_name,
        pwd => pwd,
    };

    Ok(template.render(context).unwrap())
}

pub enum ProjectState {
    New(NewProject),
    Exists(Project),
}

impl ProjectState {
    pub fn load(
        config: &config::RuntimeEnvironment,
        options: &StartArgs,
        project_name: &str,
    ) -> Result<Self> {
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Window {
    name: String,
    panels: PanelLayout,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PanelLayout {
    SinglePanel { panel: Panel },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Panel {
    command: Option<String>,
}

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
    attach: bool,
    enable_pane_titles: bool,
    pane_title_position: Option<String>,
    pane_title_format: Option<String>,
    windows: Vec<Window>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    config: Config,
    tmux_session_state: TmuxSessionState,
}

// FIXME(tatu): Reduce love of panics
fn read_yaml(content: &str) -> Config {
    let docs = YamlLoader::load_from_str(content).unwrap();

    // FIXME(tatu): verify we only have one document
    let doc = &docs.get(0).expect("configuration is empty");

    let root = Node::root(doc);

    let windows = doc["windows"]
        .as_vec()
        .expect("windows can only be a list")
        .to_vec()
        .iter()
        .map(|window| {
            // FIXME(tatu): Horrible shit and doesn't support pane configuration
            if let Some(properties) = window.as_hash() {
                let (key, value) = properties.front().unwrap();
                let panels = PanelLayout::SinglePanel {
                    panel: Panel {
                        command: value.as_str().map(String::from),
                    },
                };
                Window {
                    name: key.as_str().unwrap().to_string(),
                    panels,
                }
            } else {
                panic!("malformed configuration bro");
            }
        })
        .collect::<Vec<Window>>();

    let windows = root
        .required("windows")
        .expect("should contain window")
        .each(parse_window)
        .expect("should have parsed windows");

    Config {
        name: root
            .required_string("name")
            .expect("project should always have a name"),
        root: root
            .optional("root")
            .map(|n| n.as_string().expect("root should be a string")),
        socket_name: root
            .optional("socket_name")
            .map(|n| n.as_string().expect("socket_name should be a string")),
        on_project_start: root
            .optional("on_project_start")
            .map(|n| n.as_string().expect("on_project_start should be a string")),
        on_project_first_start: root.optional("on_project_first_start").map(|n| {
            n.as_string()
                .expect("on_project_first_start should be a string")
        }),
        on_project_restart: root.optional("on_project_restart").map(|n| {
            n.as_string()
                .expect("on_project_restart should be a string")
        }),
        on_project_exit: root
            .optional("on_project_exit")
            .map(|n| n.as_string().expect("on_project_exit should be a string")),
        on_project_stop: root
            .optional("on_project_stop")
            .map(|n| n.as_string().expect("on_project_stop should be a string")),
        pre_window: root
            .optional("pre_window")
            .map(|n| n.as_string().expect("pre_window should be a string")),
        tmux_options: root
            .optional("tmux_options")
            .map(|n| n.as_string().expect("tmux_options should be a string")),
        tmux_command: root
            .optional("tmux_command")
            .map(|n| n.as_string().expect("tmux_command should be a string")),
        startup_window: root
            .optional("startup_window")
            .map(|n| n.as_string().expect("startup_window should be a string")),
        startup_pane: root.optional("startup_pane").map(|n| {
            n.as_u64()
                .expect("startup_pane should be an unsigned integer")
        }),
        attach: root
            .optional("attach")
            .map(|n| n.as_bool().unwrap_or(true))
            .unwrap_or(true),
        enable_pane_titles: root
            .optional("enable_pane_titles")
            .map(|n| n.as_bool().unwrap_or(false))
            .unwrap_or(false),
        pane_title_position: root.optional("pane_title_position").map(|n| {
            n.as_string()
                .expect("pane_title_position should be a string")
        }),
        pane_title_format: root
            .optional("pane_title_format")
            .map(|n| n.as_string().expect("pane_title_format should be a string")),
        windows,
    }
}

fn parse_window(node: Node) -> result::Result<Window, ParseError> {
    // Each window entry is a single-key map: `- window_name: <value>`
    let (name, value) = node.as_single_entry()?;

    let panels = if value.is_str() {
        // Simple: `- editor: vim`
        PanelLayout::SinglePanel {
            panel: Panel {
                command: Some(value.as_string()?),
            },
        }
    } else if value.is_null() {
        PanelLayout::SinglePanel {
            panel: Panel { command: None },
        }
    } else {
        return Err(value.err("unexpected window value type"));
    };

    Ok(Window {
        name: name.to_string(),
        panels,
    })
}

impl Project {
    fn load_str(_: &StartArgs, contents: &str) -> Result<Self> {
        let config: Config = read_yaml(contents);

        Ok(Project {
            config,
            tmux_session_state: TmuxSessionState {
                is_new_session: false,
            },
        })
    }

    pub fn render(&self) -> Result<String> {
        render_tmux_template(&self)
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub path: String,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at {}: {}", self.path, self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Clone)]
pub struct Node<'a> {
    yaml: &'a Yaml,
    path: String,
}

impl<'a> Node<'a> {
    pub fn root(yaml: &'a Yaml) -> Self {
        Node {
            yaml,
            path: "$".into(),
        }
    }

    fn child(&self, segment: &str) -> Node<'a> {
        Node {
            yaml: &self.yaml[segment],
            path: format!("{}.{}", self.path, segment),
        }
    }

    fn err(&self, message: impl Into<String>) -> ParseError {
        ParseError {
            path: self.path.clone(),
            message: message.into(),
        }
    }

    pub fn required(&self, key: &str) -> result::Result<Node<'a>, ParseError> {
        let node = self.child(key);
        if node.yaml.is_badvalue() || node.yaml.is_null() {
            Err(self.err(format!("missing required key {}", key)))
        } else {
            Ok(node)
        }
    }

    fn index(&self, i: usize, yaml: &'a Yaml) -> Node<'a> {
        Node {
            yaml,
            path: format!("{}[{}]", self.path, i),
        }
    }

    pub fn optional(&self, key: &str) -> Option<Node<'a>> {
        let node = self.child(key);
        if node.yaml.is_badvalue() || node.yaml.is_null() {
            None
        } else {
            Some(node)
        }
    }

    fn as_u64(&self) -> result::Result<u64, ParseError> {
        self.yaml
            .as_i64()
            .and_then(|v| u64::try_from(v).ok())
            .ok_or_else(|| self.err("expected unsigned integer"))
    }

    fn as_str(&self) -> result::Result<&str, ParseError> {
        self.yaml
            .as_str()
            .ok_or_else(|| self.err("expected string"))
    }

    fn as_string(&self) -> result::Result<String, ParseError> {
        self.as_str().map(String::from)
    }

    fn as_bool(&self) -> result::Result<bool, ParseError> {
        self.yaml
            .as_bool()
            .ok_or_else(|| self.err("expected boolean"))
    }

    fn required_string(&self, key: &str) -> result::Result<String, ParseError> {
        self.required(key)?
            .yaml
            .as_str()
            .ok_or_else(|| self.err("expected string"))
            .map(String::from)
    }

    pub fn as_single_entry(&self) -> result::Result<(&'a str, Node<'a>), ParseError> {
        let hash = self
            .yaml
            .as_hash()
            .ok_or_else(|| self.err("expected map"))?;
        if hash.len() != 1 {
            return Err(self.err(format!(
                "expected single-entry map, got {} keys",
                hash.len()
            )));
        }
        let (key, value) = hash.front().unwrap();
        let key_str = key
            .as_str()
            .ok_or_else(|| self.err("map key must be a string"))?;
        let child = Node {
            yaml: value,
            path: format!("{}.{}", self.path, key_str),
        };
        Ok((key_str, child))
    }

    pub fn entries(&self) -> result::Result<Vec<(&'a str, Node<'a>)>, ParseError> {
        let hash = self
            .yaml
            .as_hash()
            .ok_or_else(|| self.err("expected map"))?;
        hash.iter()
            .map(|(k, v)| {
                let key = k
                    .as_str()
                    .ok_or_else(|| self.err("map key must be a string"))?;
                Ok((
                    key,
                    Node {
                        yaml: v,
                        path: format!("{}.{}", self.path, key),
                    },
                ))
            })
            .collect()
    }

    pub fn each<F, T>(&self, f: F) -> result::Result<Vec<T>, ParseError>
    where
        F: Fn(Node<'a>) -> result::Result<T, ParseError>,
    {
        let items = self
            .yaml
            .as_vec()
            .ok_or_else(|| self.err("expected list"))?;

        items
            .iter()
            .enumerate()
            .map(|(i, item)| f(self.index(i, item)))
            .collect()
    }

    pub fn is_str(&self) -> bool {
        self.yaml.as_str().is_some()
    }

    pub fn is_list(&self) -> bool {
        self.yaml.as_vec().is_some()
    }

    pub fn is_map(&self) -> bool {
        self.yaml.as_hash().is_some()
    }

    pub fn is_null(&self) -> bool {
        self.yaml.is_null() || self.yaml.is_badvalue()
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Project};
    use crate::{
        project::{read_yaml, Panel, PanelLayout},
        StartArgs,
    };
    use indoc::indoc;
    use pretty_assertions::assert_eq;

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

        assert!(project.is_ok(), "should be able to load project layout");

        let windows = project.unwrap().config.windows;

        assert_eq!(
            windows[0].panels,
            PanelLayout::SinglePanel {
                panel: Panel {
                    command: Some("vim -u NONE".to_string())
                }
            }
        );

        assert_eq!(
            windows[1].panels,
            PanelLayout::SinglePanel {
                panel: Panel {
                    command: Some("docker compose up --build".to_string())
                }
            }
        );

        assert_eq!(
            windows[2].panels,
            PanelLayout::SinglePanel {
                panel: Panel { command: None }
            }
        );

        assert_eq!(
            windows[3].panels,
            PanelLayout::SinglePanel {
                panel: Panel { command: None }
            }
        );
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

        for window in &project.config.windows {
            match &window.panels {
                PanelLayout::SinglePanel { panel } => {
                    assert!(panel.command.is_none(), "should be an empty command")
                }
            }
        }
    }

    // This is a breaking change from tmuxinator where these are treated as empty commands, due to
    // being values in ruby. These can be valid commands, nix LSP has a binary called 'nil'.
    #[test]
    fn it_parses_window_commands_nil_as_program() {
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

        for window in &project.config.windows {
            match &window.panels {
                PanelLayout::SinglePanel { panel } => {
                    dbg!(panel);
                    assert!(panel.command.is_some(), "should not be an empty command")
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

        let config: Config = read_yaml(yaml);

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
        assert_eq!(config.pane_title_format, Some(" [ #T ] ".to_string()));
    }

    #[test]
    fn it_renders_tmux_script() {
        let yaml = r###"# /home/somebody/.config/tmuxinator/base.yml

            name: PathOfBuilding
            root: /home/somebody/development/personal/PathOfBuilding

            pre_window: nix dev -c zsh

            windows:
            - editor: vim -u NONE
            - backend: "docker compose up --build"
            - sandbox: null
            - service: null
            "###;

        let expected = indoc! {r###"#!/usr/bin/env bash

            set -o errexit   # abort on nonzero exitstatus
            set -o pipefail  # don't hide errors within pipeset -e

            if [ -n "$DEBUG" ]; then
              set -x
            fi

            tmux start-server

            cd /home/somebody/development/personal/PathOfBuilding

            # Run project on start hooks
            # TODO(tatu): I've yet to use this

            if tmux has-session -t "PathOfBuilding" &>/dev/null; then
              # TODO(tatu): Implement 'on_project_restart'. This commands runs in the caller
              # shell before attaching to tmux on each attach after the first.
              echo "Project restart hooks not implemented!"
            else
              # XXX(tatu): Why does indentation get fucked here by extra level
              # Reset TMUX so we don't send session commands to some other session
              TMUX= tmux new-session -d -s PathOfBuilding -n editor
              tmux send-keys -t PathOfBuilding:1 cd\ /home/somebody/development/personal/PathOfBuilding C-m
              tmux send-keys -t PathOfBuilding:1 'nix dev -c zsh' C-m
              tmux send-keys -t PathOfBuilding:1 'vim -u NONE' C-m
              tmux new-window -c /home/somebody/development/personal/PathOfBuilding -t PathOfBuilding:2 -n backend
              tmux send-keys -t PathOfBuilding:2 'nix dev -c zsh' C-m
              tmux send-keys -t PathOfBuilding:2 'docker compose up --build' C-m
              tmux new-window -c /home/somebody/development/personal/PathOfBuilding -t PathOfBuilding:3 -n sandbox
              tmux send-keys -t PathOfBuilding:3 'nix dev -c zsh' C-m
              tmux new-window -c /home/somebody/development/personal/PathOfBuilding -t PathOfBuilding:4 -n service
              tmux send-keys -t PathOfBuilding:4 'nix dev -c zsh' C-m
            fi

            if [ -z "$TMUX" ]; then
              tmux -u attach-session -t PathOfBuilding
            else
              tmux -u switch-client -t PathOfBuilding
            fi
            "###
        };

        let layout_options = StartArgs {
            name: "PathOfBuilding".to_string(),
            no_attach: false,
        };

        let project = Project::load_str(&layout_options, yaml)
            .expect("should have loaded simple project for rendering");

        let output = project
            .render()
            .expect("should render simple layout without errors");

        assert_eq!(output, expected);
    }
}
