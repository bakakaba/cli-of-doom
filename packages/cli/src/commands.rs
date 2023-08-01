use clap::{Args, Subcommand};
use learn::learn;

use cli_of_doom::{fs, env};

#[derive(Subcommand)]
pub enum Commands {
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

    /// Zenu related commands
    Zenu(ZenuArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ZenuArgs {
    // Taken from https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs
    #[command(subcommand)]
    commands: ZenuCommands,
}

#[derive(Debug, Subcommand)]
enum ZenuCommands {
    Test,
}


pub fn run_command(command: Option<Commands>) {
    match command {
        Some(Commands::Test { list }) => {
            if list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Learn {}) => {
            learn::run();
        }
        Some(Commands::Ls {}) => {
            fs::dirs::list_files();
        }
        Some(Commands::Env {}) => {
            env::variables::list_variables();
        }
        Some(Commands::Zenu(zenu_args)) => {
            match zenu_args.commands {
                ZenuCommands::Test => {
                    println!("Zenu test command");
                }
            }
        }
        None => (),
    }
}