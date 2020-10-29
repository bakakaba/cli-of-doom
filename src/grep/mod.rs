use std::{error::Error, env, fs, process};

#[cfg(test)]
#[path = "./mod.tests.rs"]
mod tests;

struct SearchArgs {
    query: String,
    filename: String,
}

impl SearchArgs {
    fn new(args: &[String]) -> Result<SearchArgs, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(SearchArgs { query, filename })
    }
}

fn read_file(args: SearchArgs) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.filename)?;

    for line in search(&args.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn run() {
    let args_list: Vec<String> = env::args().collect();

    let args = SearchArgs::new(&args_list).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", args.query);
    println!("In file {}", args.filename);

    if let Err(err) = read_file(args) {
        println!("Application error: {}", err);

        process::exit(1);
    }
}
