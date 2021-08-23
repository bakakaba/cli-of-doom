use std::io::Error;

use clap::{App, Arg};
use cli_of_doom::fs::dirs;
use dotenv::dotenv;
use learn::learn;

fn create_app() -> App<'static> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    let description = env!("CARGO_PKG_DESCRIPTION");

    let app = App::new(name)
        .version(version)
        .author(authors)
        .about(description)
        .arg(
            Arg::new("configuration")
                .about("Sets a custom configuration file")
                .short('c')
                .long("cfg")
                .value_name("FILE")
                .takes_value(true),
        )
        .arg(
            Arg::new("verbosity")
                .about("Sets the level of verbosity")
                .short('v')
                .multiple_occurrences(true)
                .takes_value(true),
        );

    app
}

fn add_cmd_learn(app: App) -> App {
    app.subcommand(App::new("learn").about("Run rust examples."))
}

fn add_cmd_ls(app: App) -> App {
    app.subcommand(App::new("ls").about("List files."))
}

fn run_command(mut app: App) -> Result<(), Error> {
    let matches = app.clone().get_matches();
    println!("{:?}", matches);
    match matches.subcommand_name() {
        Some("learn") => {
            learn::run();
        }
        Some("ls") => {
            dirs::list_files();
        }
        _ => {
            app.print_long_help()?;
        }
    }

    Ok(())
}

fn main() {
    dotenv().ok();

    let mut app = create_app();
    app = add_cmd_learn(app);
    app = add_cmd_ls(app);
    run_command(app).unwrap();
}
