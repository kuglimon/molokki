use clap::{Args, Parser, Subcommand};
use std::error::Error;
use std::result;

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = config::Config::from_env()?;

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::List(args) => command::list::run(config, args.newline),
        Commands::Edit(args) => command::edit::run(config, &args.name),
        Commands::Debug(args) => command::debug::run(config, args, &args.name),
        Commands::Start(args) => command::start::run(config, args, &args.name),
    }
}
