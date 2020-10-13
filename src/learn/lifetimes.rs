use colored::Colorize;
use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn lifetimes() {
    println!("\n{}", "Lifetimes".green().bold());

    #[allow(unused_mut)]
    let mut r = 2;
    {
        #[allow(unused_variables)]
        let x = 5;
        // Borrow checker prevents this because x is dropped before r is used
        // r = &x;
    }
    println!("r: {}", r);

    /*
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
    */

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Excerpt {:?}", i);

    let s: &'static str = "I have a static lifetime.";
    println!("Static {:?}", s);
}
