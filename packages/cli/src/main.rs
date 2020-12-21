use clap::{load_yaml, App};
use dotenv::dotenv;
use learn::learn;

fn main() {
    dotenv().ok();

    // learn::run();
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    match matches.subcommand_name() {
        Some("learn") => {
            learn::run();
        }
        _ => println!("Unknown subcommand")
    }
    println!("{:?}", matches);
}
