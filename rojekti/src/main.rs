use clap::{Args, Parser, Subcommand};
use project::Project;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::DirEntry;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::{collections::BTreeMap, fs};
use std::{env, result};
use tera::{Context, Tera};

type Result<T> = result::Result<T, Box<dyn Error>>;

mod command;
mod config;
mod project;

/// Rojekti - Tmuxinator but rust
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available projects
    List(ListArgs),

    /// Start tmux session with the given project name
    Start(StartArgs),

    /// Print project template
    Debug(StartArgs),

    /// Open project config in $EDITOR
    Edit(StartArgs),
}

#[derive(Args)]
struct ListArgs {
    /// Output one project per line
    #[arg(short, long)]
    newline: bool,
}

#[derive(Args)]
pub struct StartArgs {
    /// Name of the tmux session and project
    name: String,

    /// Should we attach to the session
    #[arg(short, long)]
    no_attach: bool,
}

// TODO(tatu): Add default values
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    name: String,
    root: Option<String>,
    windows: Vec<BTreeMap<String, Option<String>>>,
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

impl TmuxScriptTemplate {
    fn build(config: Config, runtime_args: &StartArgs) -> Result<Self> {
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TmuxWindowConfig {
    name: String,
    command: String,
}

fn render_tmux_template(config: &TmuxScriptTemplate) -> Result<String> {
    // TOOD(tatu): Add proper error handling
    let mut tera = Tera::default();
    tera.add_raw_template("tmux.sh", include_str!("templates/tmux.sh"))?;
    Ok(tera.render("tmux.sh", &Context::from_serialize(&config)?)?)
}

fn write_tmux_template(s: &mut dyn Write, config: &TmuxScriptTemplate) -> Result<()> {
    // TOOD(tatu): Add proper error handling
    Ok(write!(s, "{}", render_tmux_template(config)?)?)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = config::Config::from_env()?;

    // TODO(tatu): Maybe move this under environment or some similar struct
    let home_path = env::var("HOME").expect("HOME is not set on env, cannot continue");
    // TODO(tatu): Doesn't support .config in another directory, but I never change this, meh.main
    let xdg_config_home = Path::new(&home_path).join(".config");
    let layout_home = xdg_config_home.join("tmuxinator");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::List(args) => command::list::run(args.newline),
        Commands::Edit(args) => command::edit::run(config, &args.name),
        Commands::Debug(args) => command::debug::run(config, args, &args.name),
        Commands::Start(args) => command::start::run(config, args, &args.name),
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    struct Setup;

    impl Setup {
        fn init() -> Self {
            let temp = env::temp_dir().join("rojekti-test");
            fs::create_dir_all(&temp).expect("could not create test dir");

            env::set_var(
                "XDG_CONFIG_HOME",
                temp.to_str().expect("Cannot create temp path"),
            );
            Setup {}
        }
    }

    impl Drop for Setup {
        fn drop(&mut self) {
            let temp = env::temp_dir().join("rojekti-test");
            env::remove_var("XDG_CONFIG_HOME");
            fs::remove_dir(&temp).expect("could not create test dir");
        }
    }

    #[test]
    fn it_lists_nothing_in_empty_dir() {
        let _setup = Setup::init();

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
