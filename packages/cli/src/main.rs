use std::path::PathBuf;

use clap::{Parser, Subcommand};
use dotenv::dotenv;

use cli_of_doom::fs::dirs;
use cli_of_doom::env::variables;
use learn::learn;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run learning examples
    Learn {},

    /// Rudimentary list files
    Ls {},

    /// Example subcommand with options
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },

    /// Print environment configuration
    Env {},
}

fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    match cli.verbose {
        0 => (),
        1 => println!("Verbose mode"),
        2 => println!("Extra verbose"),
        _ => println!("Don't be crazy verbose"),
    }

    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Learn {}) => {
            learn::run();
        }
        Some(Commands::Ls {}) => {
            dirs::list_files();
        }
        Some(Commands::Env {}) => {
            variables::list_variables();
        }
        None => (),
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
