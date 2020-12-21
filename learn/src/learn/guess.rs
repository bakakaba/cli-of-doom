use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn high_and_low(numbers: &str) -> String {
    println!("Numbers: {}", numbers);

    use std::cmp;
    let f = |(max, min), x| (cmp::max(max, x), cmp::min(min, x));

    let answer = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::min_value(), i32::max_value()), f);
    format!("Low: {}, High: {}", answer.0, answer.1)
}

pub fn high_and_low_example() {
    println!("\n{}", "Guess".green().bold());

    let result = high_and_low("10 20 40 50 75");
    println!("{}", result);
}

pub fn guess_number() {
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
