use clap::Subcommand;
use learn::learn;

use cli_of_doom::{env, fs};
use zenu::commands::{run_zenu_command, ZenuArgs};

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

pub async fn run_command(command: Option<Commands>) {
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
        Some(Commands::Zenu(args)) => {
            run_zenu_command(args).await;
        }
        None => (),
    }
}
