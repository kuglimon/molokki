use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::{collections::BTreeMap, fs};
use std::{env, result};
use tera::{Context, Tera};

type Result<T> = result::Result<T, Box<dyn Error>>;

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
    List {},

    /// Start tmux session with the given project name
    Start(StartArgs),

    /// Print project template
    Debug(StartArgs),

    /// Open project config in $EDITOR
    Edit(StartArgs),
}

#[derive(Args)]
struct StartArgs {
    /// Name of the tmux session and project
    #[arg(short, long)]
    name: String,

    /// Should we attach to the session
    #[arg(short, long)]
    attach: bool,
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
            attach: runtime_args.attach,
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

fn render_tmux_template(s: &mut dyn Write, config: &TmuxScriptTemplate) -> Result<()> {
    // TOOD(tatu): Add proper error handling
    let tera = match Tera::new("templates/**/*.sh") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    writeln!(s, "{:?}", &Context::from_serialize(&config)?)?;

    write!(
        s,
        "{}",
        tera.render("tmux.sh", &Context::from_serialize(&config)?)?
    )?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // TODO(tatu): Maybe move this under environment or some similar struct
    let home_path = env::var("HOME").expect("HOME is not set on env, cannot continue");
    // TODO(tatu): Doesn't support .config in another directory, but I never change this, meh.
    let xdg_config_home = Path::new(&home_path).join(".config");
    let layout_home = xdg_config_home.join("tmuxinator");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::List {} => {
            let paths = fs::read_dir(&layout_home)?;

            {
                let mut lock = io::stdout().lock();
                for path in paths {
                    write!(
                        lock,
                        " {} ",
                        path?
                            .path()
                            .with_extension("")
                            .file_name()
                            .ok_or("extension error")?
                            .to_str()
                            .ok_or("osstr error")?
                    )?
                }
            }
            Ok(())
        }
        Commands::Edit(name) => {
            let project_file = layout_home.join(&name.name).with_extension("yml");

            Command::new(
                env::var("EDITOR").expect("Broke ass environment does not have EDITOR set"),
            )
            .args([project_file.to_str().ok_or("Not a valid path")?])
            .status()?;
            Ok(())
        }
        Commands::Debug(name) => {
            let project_file = layout_home.join(&name.name).with_extension("yml");

            println!(
                "Debugging project {}",
                project_file
                    .to_str()
                    .unwrap_or("project file is not a valid path")
            );

            if project_file.is_file() {
                let contents = fs::read_to_string(project_file)
                    .expect("Could not read given project file, check permissions");

                let config: Config = serde_yaml::from_str(&contents).unwrap();

                let tmux_template = TmuxScriptTemplate::build(config, name)?;

                {
                    let mut lock = io::stdout().lock();
                    render_tmux_template(&mut lock, &tmux_template)
                }
            } else {
                println!("Given project does not exist or is not a file");
                // TODO(tatu): We should fall to create in this case
                Ok(())
            }
        }
        Commands::Start(_name) => {
            unimplemented!("Add support for start")
        }
    }
}
