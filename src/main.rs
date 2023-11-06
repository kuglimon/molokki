use clap::{Args, Parser, Subcommand};
use sailfish::TemplateOnce;

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
}

#[derive(Args)]
struct StartArgs {
    /// Name of the tmux session and project
    #[arg(short, long)]
    name: String,
}

#[derive(TemplateOnce)]
#[template(path = "tmux.stpl")]
struct HelloTemplate {
    shell: String,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::List {} => {
            println!("'myapp add' was used, name is:")
        }
        Commands::Start(name) => {
            let ctx = HelloTemplate {
                shell: "/bin/bash".to_string(),
            };
            println!("{}", ctx.render_once().unwrap());
        }
    }
}
