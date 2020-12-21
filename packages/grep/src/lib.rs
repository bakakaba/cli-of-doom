use std::{env, error::Error, fs, process};

#[cfg(test)]
#[path = "./lib.tests.rs"]
mod tests;

struct SearchArgs {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl SearchArgs {
    fn new(args: &[String]) -> Result<SearchArgs, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(SearchArgs {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn read_file(args: SearchArgs) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.filename)?;

    let results = if args.case_sensitive {
        search(&args.query, &contents)
    } else {
        search_case_insensitive(&args.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[allow(dead_code)]
pub fn run() {
    let args_list: Vec<String> = env::args().collect();

    let args = SearchArgs::new(&args_list).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", args.query);
    println!("In file {}", args.filename);

    if let Err(err) = read_file(args) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
