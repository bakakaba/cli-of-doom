// extern crate clap;

// use clap::{App, Arg};
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn _high_and_low(numbers: &str) -> String {
    use std::cmp;
    let f = |(max, min), x| (cmp::max(max, x), cmp::min(min, x));

    let answer = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::min_value(), i32::max_value()), f);
    format!("{} {}", answer.0, answer.1)
}

fn _guess_number() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please use only whole numbers.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn _variables() {
    // Mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000; // Underscore in the number is optional

    // Shadowing
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);
}

fn _data_types() -> String {
    // Variable type defines what to parse to
    let x: u32 = "42".parse().expect("Not a number!");
    println!("The value of x is: {}", x);

    // Integer literals
    let _decimal = 98_222;
    let _hex = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _byte = b'A'; // (u8 only)

    // System integer types
    let _x: isize = 10;
    let _x: usize = 10;

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    // Arrays
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let a = serde_json::to_string(&a).expect("Failed to serialize");
    println!("The value of a is: {}", a);

    a
}

fn _control() {
    // Conditions
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Ternary
    let condition = true;
    let _number = if condition { 5 } else { 6 };

    // Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // While
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // For
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn _ownership() {
    // Stack is copied, heap is moved

    // Deep clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Function ownership is the same as assignment
    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }

    let s = String::from("hello");
    let s = takes_and_gives_back(s);
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();

        (s, length)
    }

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn _references() {
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutable references
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn _slice() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let s = String::from("hello world");
    let word = first_word(&s);
    println!("first word={}", word);
}

fn _structs() {
    struct User {
        email: String,
    }

    // Immutable instance
    let user1 = User {
        email: String::from("someone@example.com"),
    };

    // Mutable instance
    let mut user2 = User {
        email: String::from("someone@example.com"),
    };

    user2.email = String::from("anotheremail@example.com");

    // Spread create
    let _user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
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

    // let result = high_and_low("10 20 40 50 75");
    // println!("{}", result);

    // guess_number()

    // variables();

    // _data_types();

    // _control();

    // _ownership();

    // _references();

    // _slice();

    _structs();
}
