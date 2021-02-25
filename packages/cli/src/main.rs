use clap::{load_yaml, App};
use dotenv::dotenv;
use learn::learn;
use cli_of_doom::fs::dirs;

fn run_command(clap: &App) {
    let cli = clap.clone();
    let matches = cli.get_matches();
    println!("{:?}", matches);
    match matches.subcommand_name() {
        Some("learn") => {
            learn::run();
        }
        Some("ls") => {
            dirs::list_files();
        }
        _ => {
            let mut cli = clap.clone();
            cli.print_long_help().unwrap();
        }
    }
}

fn main() {
    dotenv().ok();

    let yaml = load_yaml!("cli.yaml");
    let cli = App::from(yaml);
    run_command(&cli);
}
