use std::{env, fs, process};

struct SearchArgs {
    query: String,
    filename: String,
}

impl SearchArgs {
    fn new() -> Result<SearchArgs, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(SearchArgs { query, filename })
    }
}

pub fn run() {
    let args = SearchArgs::new().unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", args.query);
    println!("In file {}", args.filename);

    let contents =
        fs::read_to_string(args.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}
