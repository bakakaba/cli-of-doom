extern crate clap;

// use clap::{App, Arg};

fn high_and_low(numbers: &str) -> String {
    use std::cmp;
    let f = |(max, min), x| (cmp::max(max, x), cmp::min(min, x));

    let answer = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::min_value(), i32::max_value()), f);
    format!("{} {}", answer.0, answer.1)
}

fn main() {
    // let matches = App::new("cli-of-d00m")
    //     .version("0.1.0")
    //     .author("Lian Hoy Lee <cruz@zyk3.net>")
    //     .about("You know you're in trouble just by the name.")
    //     .arg(
    //         Arg::with_name("URL")
    //             .required(true)
    //             .takes_value(true)
    //             .index(1)
    //             .help("url to download"),
    //     )
    //     .get_matches();
    // let url = matches.value_of("URL").unwrap();
    // println!("{}", url);

    let result = high_and_low("10 20 40 50 75");
    println!("{}", result);
}
