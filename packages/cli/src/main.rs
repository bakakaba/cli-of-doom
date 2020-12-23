use clap::{load_yaml, App};
use dotenv::dotenv;
use learn::learn;

fn run_command(clap: &App) {
    let cli = clap.clone();
    let matches = cli.get_matches();
    println!("{:?}", matches);
    match matches.subcommand_name() {
        Some("learn") => {
            learn::run();
        }
        _ => {
            let mut cli = clap.clone();
            cli.print_long_help().unwrap();
        }
    }
}

fn main() {
    dotenv().ok();

    // learn::run();
    let yaml = load_yaml!("cli.yaml");
    let cli = App::from(yaml);
    run_command(&cli);
}
