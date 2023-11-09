use clap::{Args, Parser, Subcommand};
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};
use std::path::Path;
use std::{collections::BTreeMap, fs};
use std::{env, result};

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

    /// Should we attach to the session
    #[arg(short, long)]
    attach: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    name: String,
    root: Option<String>,
    windows: Vec<BTreeMap<String, Option<String>>>,
}

#[derive(TemplateOnce)]
#[template(path = "tmux.stpl", escape = false)]
struct TmuxScriptTemplate {
    config: Config,
    is_new_tmux_session: bool,
    attach: bool,
}

// I tried this with a templating engine but it printed new lines after each expression. Rather
// wasting my time figuring out how to configure it, I just went with good 'ol writeln.
//
// It's a bit harder to read, but who gives a fuck when it's so simple. This is not something I'll
// modify 20 hours a week.
fn render_tmux_template(s: &mut dyn Write, config: &TmuxScriptTemplate) -> Result<()> {
    writeln!(
        s,
        r#"#!/usr/bin/env bash

cd {root} 

# Run project on start hooks
# TODO(tatu): I've yet to use this

if tmux has-session -t "{name}" &>/dev/null; then
  # TODO(tatu): Needs window indexing"#,
        root = &config.config.root.as_ref().unwrap_or(&".".to_string()),
        name = &config.config.name
    )?;

    for window in &config.config.windows {
        let (name, _) = window
            .first_key_value()
            .expect("window does not have a valid configuration");

        writeln!(
            s,
            r#"  tmux new-window <%- "path" %> -t {target} -n {name}"#,
            target = "tmux_windows_target",
            name = name
        )?;
    }

    writeln!(
        s,
        r#"else
    echo "existing session"
    # TODO(tatu): Implement existing session support
fi
"#
    )?;

    if config.attach {
        writeln!(
            s,
            r#"if [ -z "$TMUX" ]; then
  tmux -u attach-session -t {name}
else
  tmux -u switch-client -t {name}
fi
"#,
            name = &config.config.name
        )?;
    }

    Ok(())
}

// TODO(tatu): Add support for config directory
// TODO(tatu): Add listing support
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

                let tmux_template = TmuxScriptTemplate {
                    config,
                    is_new_tmux_session: false,
                    attach: name.attach,
                };

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
