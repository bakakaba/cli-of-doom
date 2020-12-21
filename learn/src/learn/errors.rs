use colored::*;
use io::Read;
use std::{
    fs::File,
    io::{self, ErrorKind},
};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // Long version
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    f.read_to_string(&mut s)?;
    Ok(s)

    // fs::read_to_string("hello.txt")
}

pub fn errors() {
    println!("\n{}", "Errors".green().bold());

    // panic!("crash and burn");

    let f = File::open("hello.txt");

    match f {
        Ok(file) => Some(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found: {:?}", error);
                None
            }
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // File::open("hello.txt").unwrap();
    // File::open("hello.txt").expect(&format!("Failed to open {}", "hello.txt"));

    // Propogating errors
    let _result = read_username_from_file();
}
