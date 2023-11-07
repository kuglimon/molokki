use clap::{Args, Parser, Subcommand};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;
use std::{collections::BTreeMap, fs};

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

    /// Start a tmux session with the given project name
    Start(StartArgs),

    /// Prints projects template
    Debug(StartArgs),
}

#[derive(Args)]
struct StartArgs {
    /// Name of the tmux session and project
    #[arg(short, long)]
    name: String,
}

#[derive(TemplateOnce, Debug, PartialEq, Serialize, Deserialize)]
#[template(path = "tmux.stpl")]
struct Config {
    name: String,
    root: Option<String>,
    windows: Vec<BTreeMap<String, Option<String>>>,
}

// TODO(tatu): Add support for config directory
// TODO(tatu): Add listing support
fn main() {
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
            unimplemented!("Add support for listing")
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
                println!("{}", config.render_once().unwrap());
            } else {
                println!("Given project does not exist or is not a file");
                // TODO(tatu): We should fall to create in this case
            }
        }
        Commands::Start(_name) => {
            unimplemented!("Add support for start")
        }
    }
}
